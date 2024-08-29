//! # HTTP client
//!
//! High-level interface to Hyper/reqwest HTTP client.
//!
//! This library is optimized to work with Nasqueron Datasources components.

use std::collections::HashMap;
use std::io::Error as IOError;
use std::path::Path;

use lazy_static::lazy_static;
use reqwest::{Client as ReqwestClient, RequestBuilder};
use reqwest::ClientBuilder;
use reqwest::Error as ReqwestError;
use reqwest::IntoUrl;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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
     HTTP client
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// HTTP client
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new(headers: Option<HeaderMap>) -> Self {
        let client = ClientBuilder::new()
            .default_headers(build_default_headers(headers))
            .gzip(true)
            .deflate(true)
            .build()
            .expect("Can't build HTTP client");

        Self {
            client,
        }
    }

    pub async fn get<T>(&self, url: T) -> Result<Response, Error>
    where T: IntoUrl {
        let request = self.client.get(url);
        self.run(request).await
    }

    pub async fn get_with_headers<T>(&self, url: T, headers: HashMap<String, String>) -> Result<Response, Error>
    where T: IntoUrl {
        let headers = parse_headers(headers);

        let request = self.client
            .get(url)
            .headers(headers);

        self.run(request).await
    }

    pub async fn run(&self, request: RequestBuilder) -> Result<Response, Error> {
        request
            .send()
            .await
            .map_err(|error| Error::Reqwest(error))
    }

    pub async fn download<P, T>(&self, url: T, target_path: P) -> Result<usize, Error>
    where T: IntoUrl, P: AsRef<Path> {
        let mut file =  File::create(target_path)
            .await
            .map_err(|error| Error::IO(error))?;

        let mut target_content = self.get(url).await?;
        let mut bytes_read = 0;
        while let Some(chunk) = target_content
            .chunk()
            .await
            .map_err(|error| Error::Reqwest(error))?
        {
            bytes_read += file.write(chunk.as_ref())
                .await
                .map_err(|error| Error::IO(error))?;
        }

        Ok(bytes_read)
    }
}

/*   -------------------------------------------------------------
     HTTP client utilities
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub fn parse_headers(headers: HashMap<String, String>) -> HeaderMap {
    headers
        .iter()
        .map(|(name, value)| (
            name.parse().expect("Can't parse header name"),
            value.parse().expect("Can't parse header value")
        ))
        .collect()
}

fn build_default_headers(headers: Option<HeaderMap>) -> HeaderMap {
    let mut headers = headers
        .unwrap_or(HeaderMap::new());

    // RFC 7231 states User-Agent header SHOULD be sent.
    if !headers.contains_key("User-Agent") {
        headers.append("User-Agent", HeaderValue::from_static(get_user_agent()));
    }

    headers
}

/*   -------------------------------------------------------------
     HTTP client error
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

/// HTTP client error
#[derive(Debug)]
pub enum Error {
    /// Represents an underlying error from Reqwest HTTP client when processing a request.
    Reqwest(ReqwestError),

    /// Represents an IO error when doing file operations.
    IO(IOError),
}
