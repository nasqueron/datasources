//! Service to search imported FANTOIR table.
//!
//! This is intended to be exposed to the tool, and used internally to fix FANTOIR codes:
//!
//!   - the `fantoir-datasource query` command can so be used to check
//!     if an import contains expected values to validate before promotion.
//!
//!   - the Wikidata import code uses `search_fantoir_code` when the FANTOIR
//!     code doesn't contain the code direction, and it can't be computed.

use std::fmt::{Display, Formatter};
use sqlx::PgPool;

/*   -------------------------------------------------------------
     Search a fantoir code from INSEE code, identifiant communal.

     Useful to fix fantoir code from other sources.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub async fn search_fantoir_code(pool: &PgPool, code_insee: &str, identifiant_communal_voie: &str) -> Option<String> {
    sqlx::query_scalar( r#"
SELECT code_fantoir
FROM fantoir
WHERE code_insee = $1 AND identifiant_communal_voie = $2"#)
        .bind(code_insee)
        .bind(identifiant_communal_voie)
        .fetch_optional(pool)
        .await
        .unwrap()
}

/*   -------------------------------------------------------------
     Query short information about voies.

     This tool is mainly intended as an import tool, but as we need
     this query service to cross datasources, we can leverage this
     to offer a small search facility.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FantoirVoieResult {
    pub code_fantoir: String,
    pub code_insee: String,
    pub identifiant_communal_voie: String,
    pub code_nature_voie: Option<String>,
    pub libelle_voie: String,
}

impl FantoirVoieResult {
    fn get_name (&self) -> String {
        match &self.code_nature_voie {
            None => self.libelle_voie.to_string(),
            Some(kind) => match kind.len() {
                0 => self.libelle_voie.to_string(),
                _ => format!("{} {}", kind, self.libelle_voie)
            }
        }
    }
}

impl Display for FantoirVoieResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f, "{}\t{} {}\t{}",
            self.code_fantoir, self.code_insee, self.identifiant_communal_voie, self.get_name()
        )
    }
}

pub async fn query_fantoir_code(pool: &PgPool, code_fantoir: &str) -> Option<FantoirVoieResult> {
    sqlx::query_as(r#"
SELECT code_fantoir, code_insee, identifiant_communal_voie, code_nature_voie, libelle_voie
FROM fantoir
WHERE code_fantoir = $1;"#)
        .bind(code_fantoir)
        .fetch_optional(pool)
        .await
        .unwrap()
}

pub async fn query_libelle (pool: &PgPool, libelle: &str) -> Vec<FantoirVoieResult> {
    sqlx::query_as(r#"
SELECT code_fantoir, code_insee, identifiant_communal_voie, code_nature_voie, libelle_voie
FROM fantoir
WHERE libelle_voie ILIKE CONCAT('%', $1, '%');
    "#)
        .bind(libelle)
        .fetch_all(pool)
        .await
        .unwrap()
}
