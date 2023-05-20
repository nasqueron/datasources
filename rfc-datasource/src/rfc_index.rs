use std::error::Error;
use std::fs;
use std::path::Path;

static RFC_INDEX_URL: &str = "https://www.ietf.org/download/rfc-index.txt";

pub async fn get_rfc_index(source: Option<String>) -> Result<String, Box<dyn Error>> {
    match source {
        // Case 1 - A source file has been explicitly set
        Some(file) => Ok(fs::read_to_string(&file)?.parse()?),

        None => {
            if Path::new("rfc-index.txt").exists() {
                // Case 2 - The file rfc-index.txt can be found locally
                Ok(fs::read_to_string("rfc-index.txt")?.parse()?)
            } else {
                // Case 3 - Fetch the index remotely
                Ok(fetch_rfc_index().await?)
            }
        }
    }
}

async fn fetch_rfc_index() -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(RFC_INDEX_URL)
        .await?
        .text()
        .await?;

    Ok(body)
}
