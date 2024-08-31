# SPARQL client

The crate sparql-client is a SPARQL client
based on Oxigraph components.

It can be used to query SPARQL endpoints like Wikidata.

## Usage example

```
use sparql_client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("https://query.wikidata.org/sparql");
    let railway_query = r#"
#Cities connected by the Trans-Mongolian and Trans-Siberian Railway
SELECT ?city ?cityLabel ?coordinates
WHERE
{
   VALUES ?highway { wd:Q559037 wd:Q58767 }
   ?highway wdt:P2789 ?city .
    ?city wdt:P625 ?coordinates .
   SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
}
    "#;

    let solutions = client
        .query(railway_query).await
        .into_solutions()
        .expect("No response has been found for the query.");

    for city_solution in solutions {
        println!("{:?}", city_solution);
    }
}
```
