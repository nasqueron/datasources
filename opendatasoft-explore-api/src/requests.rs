//! Requests for the Opendatasoft Explore API v2.
//!
//! The `ExploreApiEndPoint` struct allows to define an API end-point, and call the Explore API,
//! ie the methods starting by /catalog in the open data portal documentation.

use reqwest::Client;
use serde::Deserialize;

use crate::ApiHttpResponse;
use crate::schema::*;

static USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// The Explore API end-point
pub struct ExploreApiEndPoint {
    /// The Opendatasoft Explore API v2 server to use
    pub url: String,

    client: Client,
}

impl ExploreApiEndPoint {
    /// Get a new instance of the API end-point, with an HTTP client ready to run queries.
    ///
    /// * `url` - The Explore API url, for example DOMAIN/api/v2/
    pub fn new (url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .expect("A HTTP client should be built"),
        }
    }

    /*  -------------------------------------------------------------
        Part 1 - catalog

        API to enumerate datasets
        - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

    /// Query catalog datasets
    pub async fn get_datasets(&self) -> DatasetsCollection {
        let url = self.get_url("/catalog/datasets");

        self.fetch(url).await
    }

    /// Export a catalog in the specified format
    ///
    /// As the method returns the raw HTTP response, you can use
    /// the chunk() method to get the next part of the export,
    /// as a Bytes object from bytes crate.
    ///
    /// * `format` - The format you want, API seems to support "json", "csv", "xls", "rdf", "ttl",
    ///              "data.json", "rss" et "dcat".
    ///
    /// Example:
    /// ```
    /// use opendatasoft_explore_api::requests::ExploreApiEndPoint;
    ///
    /// async fn print_catalog_rdf (api: ExploreApiEndPoint) {
    ///     let mut response = api.export_datasets_catalog("rdf").await;
    ///
    ///     while let Some(chunk) = response.chunk().await.unwrap() {
    ///         let bytes = chunk.to_vec(); // Vec<u8>
    ///         let text = String::from_utf8(bytes).expect("Not a valid UTF-8 bytes sequence");
    ///
    ///         print!("{}", text);
    ///     }
    ///     println!();
    /// }
    /// ```
    pub async fn export_datasets_catalog(&self, format: &str) -> ApiHttpResponse {
        let url = self
            .get_url("/catalog/exports/?")
            .replace("?", format);

        self.fetch_resource(url).await
    }

    /// List facet values
    ///
    /// Enumerate facet values for datasets and returns a list of values for each facet.
    /// Can be used to implement guided navigation in large result sets.
    pub async fn get_facets(&self) -> FacetsCollection {
        let url = self.get_url("/catalog/facets");

        self.fetch(url).await
    }

    /*  -------------------------------------------------------------
        Part 2 - datasets

        API to work on records
        - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

    /// Query datasets records
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    pub async fn get_dataset_records(&self, dataset_id: &str) -> Results {
        let url = self
            .get_url("/catalog/datasets/?/records")
            .replace("?", dataset_id);

        self.fetch(url).await
    }

    /// Export a dataset in the desired format.
    ///
    /// As the method returns the raw HTTP response, you can use
    /// the chunk() method to get the next part of the export,
    /// as a Bytes object from bytes crate.
    /// See `export_datasets_catalog` for an example.
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    /// * `format` - The format you want, API seems to support "json", "geojson", "shp", "csv",
    ///              "xls", "jsonl", "jsonld", "rdfxml", "turtle" and "n3"
    pub async fn export_dataset(&self, dataset_id: &str, format: &str) -> ApiHttpResponse {
        let url = self
            .get_url("/catalog/datasets/:id/exports/:format")
            .replace(":id", dataset_id)
            .replace(":format", format);

        self.fetch_resource(url).await
    }

    /// Show dataset information
    ///
    /// Returns a list of available endpoints for the specified dataset, with metadata and endpoints.
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    ///
    /// The response includes the following links:
    ///
    /// * the attachments endpoint
    /// * the files endpoint
    /// * the records endpoint
    /// * the catalog endpoint
    pub async fn get_dataset_information(&self, dataset_id: &str) -> Dataset {
        let mut url = self.get_url("/catalog/datasets/");
        url.push_str(dataset_id);

        self.fetch(url).await
    }

    /// List dataset facets
    ///
    /// Enumerates facet values for records and returns a list of values for each facet.
    /// Can be used to implement guided navigation in large result sets.
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    pub async fn get_dataset_facets(&self, dataset_id: &str) -> FacetsCollection {
        let url = self
            .get_url("/catalog/datasets/?/facets")
            .replace("?", dataset_id);

        self.fetch(url).await
    }

    /// List dataset attachments
    ///
    /// When a dataset is simply a collection of external files, like for FANTOIR,
    /// attachments can be the only way to know if the data has been updated, and
    /// at what URL download it.
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    pub async fn get_dataset_attachments(&self, dataset_id: &str) -> AttachmentCollection {
        let url = self
            .get_url("/catalog/datasets/?/attachments")
            .replace("?", dataset_id);

         self.fetch(url).await
    }

    /// Read a dataset record
    ///
    /// Reads a single dataset record based on its identifier.
    ///
    /// * `dataset_id` - The identifier of the dataset to be queried.
    /// * `record_id` - Record identified, for example an UUID
    pub async fn get_dataset_record(&self, dataset_id: &str, record_id: &str) -> Record {
        let url = self
            .get_url("/catalog/datasets/:id/records/:record")
            .replace(":id", dataset_id)
            .replace(":record", record_id);

        self.fetch(url).await
    }

    /*  -------------------------------------------------------------
        Helper methods
        - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

    fn get_url (&self, method: &str) -> String {
        format!("{}{}", self.url, method)
    }

    async fn fetch_resource (&self, url: String) -> ApiHttpResponse {
        self.client.get(url)
            .send().await
            .expect("Can't fetch API URL")
    }

    async fn fetch<T> (&self, url: String) -> T where for<'a> T: Deserialize<'a> {
        let body = self.fetch_resource(url).await
            .text().await
            .expect("Can't get HTTP response content");

        serde_json::from_str(&body)
            .expect("HTTP response should be a valid dataset, can't parse it.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url () {
        let endpoint = ExploreApiEndPoint::new("https://foo");
        assert_eq!("https://foo/bar", endpoint.get_url("/bar"));
        assert_eq!("https://foo", endpoint.get_url(""));
    }

    // Requests integration tests are located in tests/ folder.
}
