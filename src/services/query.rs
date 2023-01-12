//! Service to search imported FANTOIR table
//! This is intended to be exposed to the tool, and used internally to fix FANTOIR codes.

use std::fmt::{Display, Formatter};
use sqlx::{Error, FromRow, PgPool};

/*   -------------------------------------------------------------
     Search a fantoir code from INSEE code, identifiant communal.

     Useful to fix fantoir code from other sources.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub async fn search_fantoir_code(pool: &PgPool, code_insee: &str, identifiant_communal_voie: &str) -> Option<String> {
    let result = sqlx::query!(r#"
SELECT code_fantoir
FROM fantoir
WHERE code_insee = $1 AND identifiant_communal_voie = $2
    "#, code_insee, identifiant_communal_voie)
        .fetch_one(pool)
        .await;

    if let Err(Error::RowNotFound) = result {
        return None;
    }

    result.unwrap().code_fantoir
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
    let result = sqlx::query!(r#"
SELECT code_insee, identifiant_communal_voie, code_nature_voie, libelle_voie
FROM fantoir
WHERE code_fantoir = $1;
    "#, code_fantoir)
        .fetch_one(pool)
        .await;

    if let Err(Error::RowNotFound) = result {
        return None;
    }

    let result = result.unwrap();

    Some(
        FantoirVoieResult {
            code_fantoir: code_fantoir.to_string(),
            code_insee: result.code_insee.unwrap(),
            identifiant_communal_voie: result.identifiant_communal_voie.unwrap(),
            code_nature_voie: result.code_nature_voie,
            libelle_voie: result.libelle_voie.unwrap(),
        }
    )
}

pub async fn query_libelle (pool: &PgPool, libelle: &str) -> Vec<FantoirVoieResult> {
    let result = sqlx::query(r#"
SELECT code_fantoir, code_insee, identifiant_communal_voie, code_nature_voie, libelle_voie
FROM fantoir
WHERE libelle_voie ILIKE CONCAT('%', $1, '%');
    "#)
        .bind(libelle)
        .fetch_all(pool)
        .await;

    if let Err(Error::RowNotFound) = result {
        return Vec::new();
    }

    result
        .unwrap()
        .iter()
        .map(|row| FantoirVoieResult::from_row(row).unwrap())
        .collect()
}
