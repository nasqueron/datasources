//! # Utilities for database.
//!
//! This module provides helpers to interact with a PostgreSQL database.
//! Functions expect to work with an executor from sqlx crate.

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

static QUERIES_SEPARATOR: &str = "\n\n\n";

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

pub async fn run_multiple_queries(pool: &PgPool, queries: &str) {
    for query in queries.split(QUERIES_SEPARATOR) {
        sqlx::query(&query)
            .execute(pool)
            .await
            .expect("Can't run SQL query.");
    }
}
