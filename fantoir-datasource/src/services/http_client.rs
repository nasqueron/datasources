//! Build a HTTP client with proper user agent.

use std::collections::HashMap;

use ds_http_client::{parse_headers, Client};
use lazy_static::lazy_static;

/*   -------------------------------------------------------------
     HTTP client
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub fn build_http_client () -> Client {
    let mut headers = HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        get_user_agent().to_string(),
    );
    let headers = parse_headers(headers);

    Client::new(Some(headers))
}

/*   -------------------------------------------------------------
     User agent

     Compute at build time user agent to use in HTTP requests.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

lazy_static! {
    pub static ref USER_AGENT: String = format!(
        "{}/{} (https://databases.nasqueron.org/)",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
    );
}

fn get_user_agent () -> &'static str {
    &USER_AGENT
}
