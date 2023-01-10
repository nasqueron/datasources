//! Command to promote a table as the one to use.

use sqlx::PgPool;
use crate::commands::wikidata::WIKIDATA_TABLE;
use crate::db::*;

/// Promotes a FANTOIR table as the relevant version to use
pub async fn promote (fantoir_table: &str, database_url: &str) {
    let pool = connect_to_db(database_url).await;
    let queries_groups = get_queries_groups(&pool, fantoir_table).await;

    run_multiple_queries_groups(&pool, &queries_groups);
}

/// Determines the groups of queries needed for promotion
async fn get_queries_groups (pool: &PgPool, fantoir_table: &str) -> Vec<String> {
    let mut queries_groups = vec![
        include_str!("../../schema/promote/config.sql"),
        include_str!("../../schema/promote/fantoir_view.sql"),
    ];

    if is_table_exists(pool, WIKIDATA_TABLE).await {
        queries_groups.push(include_str!("../../schema/promote/wikidata.sql"));
    }

    queries_groups
        .into_iter()
        .map(|queries| queries
            .replace("/*table*/fantoir", fantoir_table)
        )
        .collect()
}
