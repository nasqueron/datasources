//! # SPARQL client

use std::collections::HashMap;
use std::io::BufRead;

use oxrdf::Term;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use sparesults::*;

use crate::services::http_client::Client as HttpClient;

type SparqlSolution = HashMap<String, Term>;

/// SPARQL client
pub struct Client {
    pub endpoint: String,
    http_client: HttpClient,
}

/// Represent results for a SPARQL query
/// A query can return a collection of solutions or a boolean.
pub enum SparqlResults {
    /// Results for SELECT queries
    Solutions(Vec<SparqlSolution>),

    /// Results for INSERT DATA, UPDATE DATA, etc. queries
    Boolean(bool),
}

impl Client {
    pub fn new (endpoint: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("Accept: application/sparql-results+xml"));

        Self {
            endpoint: String::from(endpoint),
            http_client: HttpClient::new(Some(headers)),
        }
    }

    pub async fn query (&self, query: &str) -> SparqlResults {
        let url = Url::parse_with_params(&self.endpoint, &[("query", query)])
            .expect("Can't parse endpoint as absolute URL.");

        let query_results = self.http_client.get(url).await
            .expect("Can't query endpoint")
            .text().await
            .expect("End-point didn't return a reply.");

        parse_sparql_results(&query_results)
    }
}

pub fn parse_sparql_results (query_results: &str) -> SparqlResults {
    let results_reader = get_query_results_xml_reader(query_results.as_bytes());

    SparqlResults::read(results_reader)
}

impl SparqlResults {
    pub fn read<T>(reader: QueryResultsReader<T>) -> Self
        where T: BufRead
    {
        match reader {
            QueryResultsReader::Solutions(solutions) => {
                Self::Solutions(parse_sparql_solutions(solutions))
            },
            QueryResultsReader::Boolean(bool) => Self::Boolean(bool),
        }
    }

    pub fn into_solutions (self) -> Option<Vec<SparqlSolution>> {
        match self {
            SparqlResults::Solutions(solutions) => Some(solutions),
            SparqlResults::Boolean(_) => None,
        }
    }
}

fn get_query_results_xml_reader<T>(reader: T) -> QueryResultsReader<T>
    where T: BufRead
{
    QueryResultsParser::from_format(QueryResultsFormat::Xml)
        .read_results(reader)
        .expect("Can't read SPARQL results")
}

fn parse_sparql_solutions<T> (solutions: SolutionsReader<T>) -> Vec<SparqlSolution>
    where T: BufRead
{
    solutions
        .map(|solution| {
            parse_sparql_result(
                solution.expect("Can't read solution")
            )
        })
        .collect()
}

pub fn parse_sparql_result (solution: QuerySolution) -> SparqlSolution {
    solution
        .iter()
        .map(|(variable, term)| (
            variable.as_str().to_string(),
            term.clone(),
        ))
        .collect()
}

pub fn parse_term_uri (term: &Term) -> Option<String> {
    if let Term::NamedNode(node) = term {
        Some(node.as_str().to_string())
    } else {
        None
    }
}

pub fn parse_literal (term: &Term) -> Option<String> {
    if let Term::Literal(literal) = term {
        Some(literal.value().to_string())
    } else {
        None
    }
}

pub fn is_term_empty(term: &Term) -> bool {
    match term {
        Term::NamedNode(node) => {
            // Special values IRI are considered as empty values.
            node.as_str().contains("/.well-known/genid/")
        }
        Term::BlankNode(_) => true,
        Term::Literal(_) => false,
    }
}
