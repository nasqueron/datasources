//! # Utilities for database.
//!
//! This module provides helpers to interact with a PostgreSQL database.
//! Functions expect to work with an executor from sqlx crate.

use std::future::Future;

use async_scoped::TokioScope;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

static QUERIES_SEPARATOR: &str = "\n\n\n";

pub struct TableInitializationArgs {
    pub table_name: String,
    pub create_table: bool,
    pub overwrite_table: bool,
}

pub trait ToTableInitializationArgs {
    fn to_table_initialization_args(&self) -> TableInitializationArgs;
}

pub async fn connect_to_db (database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(3)
        .connect(database_url)
        .await
        .expect("Can't connect to database.")
}

pub async fn is_table_exists (pool: &PgPool, table: &str) -> bool {
    let query = r#"
    SELECT EXISTS (
               SELECT FROM
                   pg_tables
               WHERE
                       schemaname = 'public' AND
                       tablename  = $1
           );
    "#;

    let result: (bool,) = sqlx::query_as(query)
        .bind(table)
        .fetch_one(pool)
        .await
        .expect("Can't check if table exists.");

    result.0
}

pub async fn is_table_empty (pool: &PgPool, table: &str) -> bool {
    let query = r#"
    SELECT EXISTS (
        SELECT 1 FROM %%table%%
    );
    "#.replace("%%table%%", table);

    let result: (bool,) = sqlx::query_as(&query)
        .fetch_one(pool)
        .await
        .expect("Can't check if table is empty.");

    !result.0
}

pub async fn truncate_table (pool: &PgPool, table: &str) {
    let query = format!("TRUNCATE TABLE {} RESTART IDENTITY;", table);

    sqlx::query(&query)
        .bind(table)
        .execute(pool)
        .await
        .expect("Can't truncate table.");
}

pub async fn initialize_table<F, T> (
    pool: &PgPool,
    callback: F,
    args: T
) -> Result<(), String>
    where F: Future, T: ToTableInitializationArgs
{
    let args = args.to_table_initialization_args();
    if is_table_exists(pool, &args.table_name).await {
        if is_table_empty(&pool, &args.table_name).await {
            return Ok(());
        }

        if args.overwrite_table {
            truncate_table(&pool, &args.table_name).await;
            return Ok(());
        }

        return Err(format!(
            "Table {} already exists and contains rows. To overwrite it, run the import tool with -t option.",
            &args.table_name
        ));
    }

    if args.create_table {
        callback.await;
        return Ok(());
    }

    Err(format!(
        "Table {} doesn't exist. To create it, run the import tool with -c option.",
        &args.table_name
    ))
}

pub async fn run_multiple_queries(pool: &PgPool, queries: &str) {
    for query in queries.split(QUERIES_SEPARATOR) {
        sqlx::query(&query)
            .execute(pool)
            .await
            .expect("Can't run SQL query.");
    }
}

pub fn run_multiple_queries_groups (pool: &PgPool, queries_groups: &Vec<String>) {
    let n = queries_groups.len();
    TokioScope::scope_and_block(|scope| {
        for i in 0..n {
            scope.spawn(
                run_multiple_queries(pool, &queries_groups[i])
            )
        }
    });
}
