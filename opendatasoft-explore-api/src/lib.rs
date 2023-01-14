//! `opendatasoft_explore_api` is an Opendatasoft Explore API v2 client library.
//!
//! It allows to query open data portals from public administrations and companies
//! to get information about datasets metadata and records.
//!
//! # Example
//!
//! The library is compatible with data.economie.gouv.fr, the "portail des données ouvertes"
//! from the France agency for economy. Let's query information about their FANTOIR file,
//! a database with streets, neighborhoods or private residence names.
//!
//! ```rust,no_run
//! use opendatasoft_explore_api::requests::ExploreApiEndPoint;
//!
//! static API_URL: &'static str = "https://data.economie.gouv.fr/api/v2";
//! static DATASET_ID: &'static str = "fichier-fantoir-des-voies-et-lieux-dits";
//!
//! #[tokio::main]
//! async fn main() {
//!     let endpoint = ExploreApiEndPoint::new(API_URL);
//!
//!     let dataset = endpoint.get_dataset_information(DATASET_ID).await;
//!     println!("{:?}", dataset);
//! }
//! ```
//!
//! # Asynchronous code
//!
//! The library uses under the hood Reqwest to perform async HTTP calls.
//!
//! Our code is tested with Tokio, but you can use the async runtime of your choice.
//!
//! # Under the hood
//!
//! Reqwest is used to run queries as HTTP client.
//!
//! Serde converts JSON responses into the structures defined in schema module.
//!
//! # Library organization
//!
//! The crate offers is organization in two modules:
//!
//! * In requests module, the [`ExploreApiEndPoint`](./requests/struct.ExploreApiEndPoint.html)
//!   allows to prepare an HTTP client and define the end-point API URL;
//!
//! * In schema module, the structs represent datatypes used by the API responses.
//!
//! The requests are documented in `ExploreApiEndPoint`. From there, you'll always have a link
//! to the schema used, as the return type of the method.

pub mod schema;
pub mod requests;

pub use reqwest::Response as ApiHttpResponse;
