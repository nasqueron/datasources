use std::io::Error as IOError;
use std::path::Path;

use lazy_static::lazy_static;
use reqwest::Client as ReqwestClient;
use reqwest::ClientBuilder;
use reqwest::Error as ReqwestError;
use reqwest::IntoUrl;
use reqwest::Response;
use reqwest::header::HeaderMap;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/*   -------------------------------------------------------------
     User agent

     The USER_AGENT variable is computed at build time.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

lazy_static! {
    pub static ref USER_AGENT: String = format!(
        "{}/{} (https://databases.nasqueron.org/)",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
    );
}

pub fn get_user_agent () -> &'static str {
    &USER_AGENT
}

/*   -------------------------------------------------------------
     HTTP client
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new(headers: Option<HeaderMap>) -> Self {
        let headers = headers
            .unwrap_or(HeaderMap::new());

        let client = ClientBuilder::new()
            .user_agent(get_user_agent())
            .default_headers(headers)
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
        self.client
            .get(url)
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
