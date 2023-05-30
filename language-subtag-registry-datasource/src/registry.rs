use std::error::Error;
use std::fs;
use std::path::Path;

use reqwest::ClientBuilder;

static REGISTRY_URL: &str = "https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry";

/*   -------------------------------------------------------------
     User agent

     The USER_AGENT variable is computed at build time.
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

lazy_static::lazy_static! {
    pub static ref USER_AGENT: String = format!(
        "{}/{} (https://databases.nasqueron.org/)",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
    );
}

pub fn get_user_agent () -> &'static str {
    &USER_AGENT
}

/*   -------------------------------------------------------------
     Read or fetch registry
     - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

pub async fn get_registry(source: Option<String>) -> Result<String, Box<dyn Error>> {
    match source {
        // Case 1 - A source file has been explicitly set
        Some(file) => Ok(fs::read_to_string(&file)?.parse()?),

        None => {
            if Path::new("registry.txt").exists() {
                // Case 2 - The file registry.txt can be found locally
                Ok(fs::read_to_string("registry.txt")?.parse()?)
            } else {
                // Case 3 - Fetch the index remotely
                Ok(fetch_registry().await?)
            }
        }
    }
}

async fn fetch_registry() -> Result<String, Box<dyn Error>> {
    let client = ClientBuilder::new()
        .user_agent(get_user_agent())
        .gzip(true)
        .deflate(true)
        .build()
        .expect("Can't build HTTP client");

    let body = client.get(REGISTRY_URL)
        .send().await?
        .text().await?;

    Ok(body)
}
