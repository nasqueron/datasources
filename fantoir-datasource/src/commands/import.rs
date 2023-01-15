//! Import command for the fantoir-datasource tool.
//!
//! Import from FANTOIR file generated by the DGFIP

use std::process::exit;

use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::ImportArgs;
use crate::db::*;
use crate::fantoir::FantoirEntry;

impl ToTableInitializationArgs for &ImportArgs {
    fn to_table_initialization_args (&self) -> TableInitializationArgs {
        TableInitializationArgs {
            table_name: self.fantoir_table.clone(),
            create_table: self.create_table,
            overwrite_table: self.overwrite_table,
        }
    }
}

async fn create_table(pool: &PgPool, table: &str) {
    let queries = include_str!("../schema/fantoir.sql")
        .replace("/*table*/fantoir", table)
        .replace("/*index*/index_fantoir_", format!("index_{}_", table).as_ref());

    run_multiple_queries(pool, &queries).await;
}

pub async fn import(args: &ImportArgs, database_url: &str) {
    let fd = File::open(&args.fantoir_file).await.expect("Can't open file.");
    let pool  = connect_to_db(database_url).await;

    // Create/truncate table as needed and as allowed by options
    let callback = async {
        create_table(&pool, &args.fantoir_table).await;
    };
    if let Err(error) = initialize_table(&pool, callback, args).await {
        eprintln!("{}", &error);
        exit(1);
    }

    // Currently, async closures are unstable, see https://github.com/rust-lang/rust/issues/62290
    // They are also largely unimplemented. As such, this code doesn't follow HOF pattern.
    let mut buffer = BufReader::new(fd).lines();
    while let Ok(line) = buffer.next_line().await {
        if line.is_none() {
            break;
        }
        let line = line.unwrap();

        if line.len() < 90 {
            // This record is the header or describes a department or a commune
            continue;
        }

        if line.starts_with("9999999999") {
            // This record is the last of the database
            break;
        }

        FantoirEntry::parse_line(&line)
            .insert_to_db(&pool, &args.fantoir_table)
            .await
    }
}