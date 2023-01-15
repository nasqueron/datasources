//! Query Wikidata SPARQL end-point and import result into PostgreSQL

mod qualification;
mod report;

use std::collections::HashMap;
use std::process::exit;

use oxrdf::Term;
use sqlx::PgPool;

use crate::commands::wikidata::qualification::determine_p31_winner;
use crate::commands::wikidata::report::*;
use crate::db::*;
use crate::WikidataArgs;
use crate::fantoir::{fix_fantoir_code, FixedFantoirCode};
use crate::services::query::search_fantoir_code;
use crate::services::sparql::*;

pub static WIKIDATA_TABLE: &'static str = "fantoir_wikidata";
pub static WIKIDATA_SPARQL_ENDPOINT: &'static str = "https://query.wikidata.org/sparql";

/*   -------------------------------------------------------------
     Import task
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub async fn import (args: &WikidataArgs, database_url: &str) {
    let pool = connect_to_db(database_url).await;

    // Create/truncate table as needed and as allowed by options
    let callback = async {
        let queries = include_str!("../../schema/wikidata.sql");
        run_multiple_queries(&pool, &queries).await;
    };
    if let Err(error) = initialize_table(&pool, callback, args).await {
        eprintln!("{}", &error);
        exit(1);
    }

    // Query Wikidata and get (Wikidata/FANTOIR code, list of P31 (instance of) values) hashmap
    let client = Client::new(WIKIDATA_SPARQL_ENDPOINT);
    let mut what_map = HashMap::new();

    client.query(include_str!("../../queries/wikidata.sparql"))
        .await
        .into_solutions()
        .expect("A list of solutions is expected for a SELECT query")
        .iter()
        .filter(|entry| !is_term_empty(&entry["code_fantoir"]))
        .for_each(|entry| {
            // Build a map of the different P31 (instance of) values for a specified code.

            let key = WikidataEntryKey::parse(entry);
            let what = parse_wikidata_entity_uri(&entry["what"]).expect("Can't parse P31 what result");

            what_map.entry(key).or_insert(Vec::new())
                .push(what);
        });

    // Consolidate entries and insert them into the database.
    // To avoid an async closure, we don't use HOF pattern.
    let mut maintenance_report = HashMap::new();
    for (key, candidates) in what_map {
        if let Some(entry) = WikidataEntry::consolidate_set(&pool, &key, candidates).await {
            if let Err(error) = entry.insert_to_db(&pool).await {
                if args.maintenance_report {
                    update_report(&mut maintenance_report, key, error);
                } else {
                    eprintln!();
                    eprintln!("Can't insert Wikidata information for the following entry:");
                    eprintln!("{:?}", entry);
                    eprintln!("{}", error);
                }
            }
            continue;
        }

        if args.maintenance_report {
            let entry = maintenance_report
                .entry("Can't resolve FANTOIR code")
                .or_insert(Vec::new());
            entry.push(key);
        } else {
            eprintln!();
            eprintln!("Can't insert Wikidata information for the following entry:");
            eprintln!("{:?}", &key);
            eprintln!("Can't resolve FANTOIR code.");
        }
    }

    if args.maintenance_report {
        print_maintenance_report(maintenance_report);
    }
}

/*   -------------------------------------------------------------
     Arguments parsing
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

impl ToTableInitializationArgs for &WikidataArgs {
    fn to_table_initialization_args(&self) -> TableInitializationArgs {
        TableInitializationArgs {
            table_name: String::from(WIKIDATA_TABLE),
            create_table: self.create_table,
            overwrite_table: self.overwrite_table,
        }
    }
}

/*   -------------------------------------------------------------
     Wikidata entry structures

     WikidataEntry represents the data ready to be inserted
     in our database.

     WikidataEntryKey is a subset of WikidataEntry to identify
     a set (FANTOIR code, Wikidata item) to be used as HashMap key
     when a SPARQL query returns several rows for such set.

     For example, here, we ask for P31 values, and if a Wikidata
     entity offers several P31 values, we'll get one row per value.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[derive(Debug, Clone)]
struct WikidataEntry {
    code_fantoir: String,
    code_fantoir_wikidata: String,
    item: String,
    item_label: String,
    what: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WikidataEntryKey {
    pub code_fantoir_wikidata: String,
    pub item: String,
    pub item_label: String,
}

impl WikidataEntryKey {
    fn parse(entry: &HashMap<String, Term>) -> Self {
        Self {
            code_fantoir_wikidata: parse_literal(&entry["code_fantoir"]).expect("Can't parse code"),
            item: parse_wikidata_entity_uri(&entry["item"]).expect("Can't parse item"),
            item_label: parse_literal(&entry["itemLabel"]).expect("Can't parse item label"),
        }
    }
}

impl WikidataEntry {
    async fn consolidate_set(pool: &PgPool, key: &WikidataEntryKey, what_candidates: Vec<String>) -> Option<Self> {
        let what = determine_p31_winner(&what_candidates);

        let code_fantoir = match fix_fantoir_code(&key.code_fantoir_wikidata) {
            FixedFantoirCode::Computed(code) => code,
            FixedFantoirCode::ToSearch { code_insee, identifiant_communal_voie } => {
                search_fantoir_code(pool, &code_insee, &identifiant_communal_voie).await?
            }
        };

        Some(Self {
            code_fantoir,
            code_fantoir_wikidata: key.code_fantoir_wikidata.clone(),
            item: key.item.clone(),
            item_label: key.item_label.clone(),
            what,
        })
    }

    async fn insert_to_db (&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let mut query = format!("INSERT INTO {}", WIKIDATA_TABLE);
        query.push_str(
            r#"
            (code_fantoir, code_fantoir_wikidata, item, item_label, what)
        VALUES
            ($1, $2, $3, $4, $5)"#
        );

        sqlx::query(&query)
            .bind(&self.code_fantoir)
            .bind(&self.code_fantoir_wikidata)
            .bind(&self.item)
            .bind(&self.item_label)
            .bind(&self.what)

            .execute(pool)
            .await
            .map(|_result| ())
    }
}

/*   -------------------------------------------------------------
     Wikidata helper methods
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Parses a Wikidata entity URI from a RDF term.
///
/// For example, to parse a term representing Q1234:
///
/// ```
/// let term = Term::NamedNode(
///     NamedNode::new("http://www.wikidata.org/entity/Q1234").unwrap()
/// );
/// let entity = parse_wikidata_entity_uri(&term).unwrap();
///
/// assert_eq!("Q1234", &entity);
/// ```
pub fn parse_wikidata_entity_uri (term: &Term) -> Option<String> {
    parse_term_uri(term)
        .map(|uri| {
            let pos = uri.rfind('/').expect("URI doesn't contain any /") + 1;

            uri[pos..].to_string()
        })
}

/*   -------------------------------------------------------------
     Tests
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[cfg(test)]
mod tests {
    use oxrdf::NamedNode;
    use super::*;

    #[test]
    pub fn test_parse_wikidata_entity_uri () {
        let node = NamedNode::new("http://www.wikidata.org/entity/Q849777").unwrap();
        let term = Term::NamedNode(node);

        assert_eq!("Q849777", &parse_wikidata_entity_uri(&term).unwrap());
    }
}
