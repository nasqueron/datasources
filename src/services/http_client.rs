use lazy_static::lazy_static;

use reqwest::{Client as ReqwestClient, ClientBuilder, Error, IntoUrl, Response};
use reqwest::header::HeaderMap;

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
    }
}
