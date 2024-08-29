//! # SPARQL client

use std::collections::HashMap;
use std::io::BufRead;

use ds_http_client::Client as HttpClient;
use lazy_static::lazy_static;
use oxrdf::Term;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;
use sparesults::*;

type SparqlSolution = HashMap<String, Term>;

/*   -------------------------------------------------------------
     SPARQL client
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// SPARQL client
pub struct Client {
    pub endpoint: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new (endpoint: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static(get_user_agent()));
        let http_client = HttpClient::new(Some(headers));

        Self::from_http_client(endpoint, http_client)
    }

    pub fn from_http_client(endpoint: &str, http_client: HttpClient) -> Self {
        Self {
            endpoint: String::from(endpoint),
            http_client,
        }
    }

    pub async fn query (&self, query: &str) -> SparqlResults {
        let url = Url::parse_with_params(&self.endpoint, &[("query", query)])
            .expect("Can't parse endpoint as absolute URL.");
        let headers = self.get_query_headers();

        let query_results = self.http_client
            .get_with_headers(url, headers).await
            .expect("Can't query endpoint")
            .text().await
            .expect("End-point didn't return a reply.");

        parse_sparql_results(&query_results)
    }

    fn get_query_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "Accept: application/sparql-results+xml".to_string());

        headers
    }
}

/*   -------------------------------------------------------------
     SPARQL query results
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// Represent results for a SPARQL query
/// A query can return a collection of solutions or a boolean.
pub enum SparqlResults {
    /// Results for SELECT queries
    Solutions(Vec<SparqlSolution>),

    /// Results for INSERT DATA, UPDATE DATA, etc. queries
    Boolean(bool),
}

impl SparqlResults {
    pub fn read<T>(reader: QueryResultsReader<T>) -> Self
    where
        T: BufRead
    {
        match reader {
            QueryResultsReader::Solutions(solutions) => {
                Self::Solutions(parse_sparql_solutions(solutions))
            },
            QueryResultsReader::Boolean(bool) => Self::Boolean(bool),
        }
    }

    pub fn into_solutions(self) -> Option<Vec<SparqlSolution>> {
        match self {
            SparqlResults::Solutions(solutions) => Some(solutions),
            SparqlResults::Boolean(_) => None,
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        match self {
            SparqlResults::Solutions(_) => None,
            SparqlResults::Boolean(bool) => Some(bool),
        }
    }
}

pub fn parse_sparql_results (query_results: &str) -> SparqlResults {
    let results_reader = get_query_results_xml_reader(query_results.as_bytes());

    SparqlResults::read(results_reader)
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

/*   -------------------------------------------------------------
     Helper methods
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub fn is_term_empty(term: &Term) -> bool {
    match term {
        Term::NamedNode(node) => {
            // Special values IRI are considered as empty values.
            node.as_str().contains("/.well-known/genid/")
        }
        Term::BlankNode(_) => true,
        Term::Literal(_) => false,
        Term::Triple(triple) => is_term_empty(&triple.object),
    }
}

/*   -------------------------------------------------------------
     User agent

     The USER_AGENT variable is computed at build time.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

lazy_static! {
    pub static ref USER_AGENT: String = format!(
        "{}/{}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
    );
}

/// Gets the default user agent
pub fn get_user_agent () -> &'static str {
    &USER_AGENT
}

/*   -------------------------------------------------------------
     Tests
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_solution_results() {
        let solutions_result = r#"
<?xml version="1.0"?>
<sparql xmlns="http://www.w3.org/2005/sparql-results#">
  <head>
    <variable name="foo"/>
  </head>
  <results>
    <result>
      <binding name="foo">
        <literal xml:lang="en">bar</literal>
      </binding>
    </result>
  </results>
</sparql>
        "#;

        let results = parse_sparql_results(solutions_result);
        let actual = results.into_solutions();

        assert!(actual.is_some());

        let solutions = actual.unwrap();
        assert_eq!(1, solutions.iter().count());

        let solution = solutions.first().unwrap();
        assert_eq!(1, solution.iter().count());

        // Asserts solution can be parsed as foo=bar
        assert!(solution.contains_key("foo"));
        let term = &solution["foo"];
        assert!(term.is_literal());
        let actual = parse_literal(term).unwrap();
        assert!(actual.eq("bar"));
    }

    #[test]
    pub fn test_parse_boolean_results () {
        let boolean_results = r#"
<?xml version="1.0"?>
<sparql xmlns="http://www.w3.org/2005/sparql-results#">
  <head />
  <boolean>true</boolean>
</sparql>
        "#;

        let results = parse_sparql_results(boolean_results);
        let actual = results.into_bool();

        assert!(actual.is_some());
        assert!(actual.unwrap());
    }
}
