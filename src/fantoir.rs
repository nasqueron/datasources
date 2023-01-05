//! # Helper methods for FANTOIR database.
//!
//! This module offers a structure for a FANTOIR record, methods to parse the file and export it.
//! Database functions expect to work with an executor from sqlx crate.

use sqlx::PgPool;
use sqlx::types::chrono::NaiveDate;

/// A voie in the FANTOIR database
#[derive(Debug)]
pub struct FantoirEntry {
    /* Identifiers */
    code_fantoir: String,

    /* Part 1 - commune */
    departement: String, // Generally an integer, but INSEE uses 2A and 2B for Corse
    code_commune: i32,
    code_insee: String,  // Afa in Corse has 2A001
    type_commune: Option<String>,
    is_pseudo_recensee: bool,

    /* Part 2 - voie */
    identifiant_communal_voie: String,
    cle_rivoli: String,
    code_nature_voie: Option<String>,
    libelle_voie: String,
    type_voie: i32, // 1: voie, 2: ens. immo, 3: lieu-dit, 4: pseudo-voie, 5: provisoire
    is_public: bool,

    /* Part 3 - population */
    is_large: bool,
    population_a_part: i32,
    population_fictive: i32,

    /* Part 4 - metadata */
    is_cancelled: bool,
    cancel_date: Option<NaiveDate>,
    creation_date: Option<NaiveDate>,
    code_majic: i32,
    last_alpha_word: String,
}

impl FantoirEntry {
    pub fn parse_line(line: &str) -> Self {
        let departement = match &line[0..2] {
            "97" => String::from(&line[0..3]), // include for DOM/TOM the next digit
            department => String::from(department),
        };
        let len = line.len();

        Self {
            /* Identifier */
            code_fantoir: String::from(&line[0..11]),

            /* Part 1 - commune */
            departement,
            code_commune: line[3..6].parse().expect("Can't parse code commune"),
            code_insee: format!("{:02}{:03}", &line[0..2], &line[3..6]),
            type_commune: parse_optional_string(&line[43..44]),
            is_pseudo_recensee: &line[45..46] == "3",

            /* Part 2 - voie */
            identifiant_communal_voie: String::from(&line[6..10]),
            cle_rivoli: String::from(&line[10..11]),
            code_nature_voie: parse_optional_string(&line[11..15]),
            libelle_voie: String::from(line[15..41].trim()),
            type_voie: line[108..109].parse().expect("Can't parse type de voie."),
            is_public: &line[48..49] == "0",

            /* Part 3 - population */
            is_large: &line[49..50] == "*",
            population_a_part: line[59..66].parse().expect("Can't parse population à part"),
            population_fictive: line[66..73].parse().expect("Can't parse population fictive"),

            /* Part 4 - metadata */
            is_cancelled: &line[73..74] != " ",
            cancel_date: parse_fantoir_date(&line[74..81]),
            creation_date: parse_fantoir_date(&line[81..88]),
            code_majic: line[103..108].parse().expect("Can't parse MAJIC"),
            last_alpha_word: String::from(&line[112..len]),
        }
    }

    pub async fn insert_to_db(&self, pool: &PgPool, table: &str) {
        let mut query = format!("INSERT INTO {}", table);
        query.push_str(
            r#"
            (code_fantoir,
             departement, code_commune, code_insee, type_commune, is_pseudo_recensee,
             identifiant_communal_voie, cle_rivoli, code_nature_voie, libelle_voie, type_voie, is_public,
             is_large, population_a_part, population_fictive,
             is_cancelled, cancel_date, creation_date, code_majic, last_alpha_word
            )
        VALUES
            ($1,
             $2, $3, $4, $5, $6,
             $7, $8, $9, $10, $11, $12,
             $13, $14, $15,
             $16, $17, $18, $19, $20
            )"#
        );

        sqlx::query(&query)
            /* Identifiers */
            .bind(&self.code_fantoir)

            /* Part 1 - commune */
            .bind(&self.departement)
            .bind(&self.code_commune)
            .bind(&self.code_insee)
            .bind(&self.type_commune)
            .bind(&self.is_pseudo_recensee)

            /* Part 2 - Voie */
            .bind(&self.identifiant_communal_voie)
            .bind(&self.cle_rivoli)
            .bind(&self.code_nature_voie)
            .bind(&self.libelle_voie)
            .bind(&self.type_voie)
            .bind(&self.is_public)

            /* Part 3 - Population */
            .bind(&self.is_large)
            .bind(&self.population_a_part)
            .bind(&self.population_fictive)

            /* Part 4 - Metadata */
            .bind(&self.is_cancelled)
            .bind(&self.cancel_date)
            .bind(&self.creation_date)
            .bind(&self.code_majic)
            .bind(&self.last_alpha_word)

            .execute(pool)
            .await
            .expect("Can't insert entry to database");
    }
}

pub fn parse_fantoir_date (date: &str) -> Option<NaiveDate> {
    if date == "0000000" {
        return None;
    }

    let year = date[0..4].parse().expect("Can't parse date: year part");
    let ord = date[4..7].parse().expect("Can't parse date: ordinal part");

    NaiveDate::from_yo_opt(year, ord)
}

fn parse_optional_string (expression: &str) -> Option<String> {
    let expression = expression.trim();

    if expression.len() > 0 {
        Some(String::from(expression))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_fantoir_date() {
        let expected = NaiveDate::from_ymd_opt(1987, 1, 1).unwrap();
        let actual = parse_fantoir_date("1987001");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_optional_string() {
        assert_eq!(Some(String::from("quux")), parse_optional_string("quux"));
    }

    #[test]
    fn test_parse_optional_string_with_trailing_spaces() {
        assert_eq!(Some(String::from("quux")), parse_optional_string("quux    "));
    }

    #[test]
    fn test_parse_optional_string_when_empty() {
        assert_eq!(true, parse_optional_string("").is_none());
    }

    #[test]
    fn test_parse_optional_string_when_only_spaces() {
        assert_eq!(true, parse_optional_string("    ").is_none());
    }
}
