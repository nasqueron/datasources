use std::process::exit;

use sqlx::PgPool;

use crate::db::connect_to_db;
use crate::QueryArgs;
use crate::services::query::*;

static EXIT_CODE_NO_RESULT_FOUND: i32 = 4;

pub async fn search(args: QueryArgs, database_url: &str) {
    let pool  = connect_to_db(database_url).await;

    if args.code_insee.is_some() && args.code_voie.is_some() {
        let code_fantoir = search_fantoir_code(
            &pool,
            &args.code_insee.unwrap(),
            &args.code_voie.unwrap(),
        ).await;

        if let Some(code) = code_fantoir {
            search_one_row(&pool, &code).await;
            return;
        }

        exit(EXIT_CODE_NO_RESULT_FOUND);
    }

    if args.libelle.len() > 0 {
        search_libelle(&pool, args).await;
        return;
    }

    unimplemented!()
}

async fn search_one_row(pool: &PgPool, code_fantoir: &str) {
    match query_fantoir_code(pool, code_fantoir).await {
        None => {
            exit(EXIT_CODE_NO_RESULT_FOUND);
        }
        Some(result) => {
            println!("{}", result);
        }
    }
}

async fn search_libelle(pool: &PgPool, args: QueryArgs) {
    let expression = args.libelle.join(" ");

    let mut found = false;

    query_libelle(pool, &expression)
        .await
        .iter()
        .filter(|&entry| entry_matches_conditions(entry, &args))
        .for_each(|entry| {
            found = true;

            println!("{}", entry);
        });

    if !found {
        exit(EXIT_CODE_NO_RESULT_FOUND);
    }
}

fn entry_matches_conditions(entry: &FantoirVoieResult, conditions: &QueryArgs) -> bool {
    if let Some(code_insee) = &conditions.code_insee {
        if &entry.code_insee != code_insee {
            return false;
        }
    }

    return true;
}
