//! Fetch command for the fantoir-datasource tool.
//!
//! Check last version and download if needed

use std::env;
use std::path::PathBuf;
use std::process::exit;

use chrono::Utc;
use tokio::fs::remove_file;
use opendatasoft_explore_api::requests::ExploreApiEndPoint;
use tokio::process::Command;

use crate::commands::fetch::fantoir_file::FantoirFile;
use crate::commands::fetch::os::is_command_available;
use crate::services::http_client::build_http_client;

mod fantoir_file;
mod os;

static ENDPOINT: &'static str = "https://data.economie.gouv.fr/api/v2";
static DATASET_ID: &'static str = "fichier-fantoir-des-voies-et-lieux-dits";

pub async fn fetch (overwrite: bool) {
    let fantoir_file = get_last_file_information().await;

    let file_exists = fantoir_file.exists_locally();
    if file_exists && !overwrite {
        eprintln!("FANTOIR file already exists. Run with --overwrite to overwrite it.");
        exit(12);
    }

    if !is_command_available("unzip") {
        eprintln!("No 'unzip' utility has been found, please install it or fix PATH if needed.");
        exit(32);
    }

    let target_path = get_fantoir_zip_path();
    if let Err(error) = build_http_client().download(&fantoir_file.url, &target_path).await {
        eprintln!("Can't download FANTOIR file: {:?}", error);
        exit(16);
    }

    let exit_code = match unzip(&target_path, overwrite).await {
        Ok(path) => {
            println!("FANTOIR_FILE={}", &path);
            println!("FANTOIR_TABLE={}", suggest_fantoir_table(&path));

            0
        }
        Err(exit_code) => exit_code,
    };

    if let Err(error) = remove_file(&target_path).await {
        eprintln!("Can't remove downloaded temporary file: {}", error);
        eprintln!("Please delete manually {}", target_path.to_str().unwrap())
    }

    exit(exit_code);
}

/// Suggests a FANTOIR table name based on the file version
fn suggest_fantoir_table(filename: &str) -> String {
    assert_eq!(11, filename.len(), "Fantoir filename is expected to have 11 characters.");

    let month: i8 = filename[7..=8].parse().unwrap();
    let year = 2000 + filename[9..=10].parse::<i32>().unwrap();

    format!("fantoir_{}{:02}", year, month)
}

/// Determines a temporary location where to save the FANTOIR file ZIP archive
fn get_fantoir_zip_path() -> PathBuf {
    let filename = format!("fantoir-download-{}.zip", Utc::now().timestamp());

    env::temp_dir()
        .join(filename)
}

async fn unzip(archive_path: &PathBuf, overwrite: bool) -> Result<String, i32> {
    let overwrite_option = match overwrite {
        true => "-o",
        false => "-n",
    };

    let process = Command::new("unzip")
        .arg(overwrite_option)
        .arg(archive_path.as_path().to_str().unwrap())
        .output()
        .await
        .expect("Can't spawn unzip process");

    if process.status.success() {
        match find_extracted_file(process.stdout) {
            None => Err(127),
            Some(filename) => Ok(filename),
        }
    } else {
        Err(process.status.code().unwrap())
    }
}

fn find_extracted_file(stdout: Vec<u8>) -> Option<String> {
    let output = String::from_utf8(stdout)
        .expect("Can't read unzip stdout");

    for action in vec!["inflating: ", "extracting: "] {
        if !output.contains(action) {
            continue;
        }

        let pos = output.find(action).unwrap() + action.len();
        let buffer = &output[pos..];
        let pos = buffer.find("\r\n").unwrap_or(
            buffer.find("\n").unwrap()
        );

        return Some(String::from(buffer[..pos].trim()));
    }

    None
}

pub async fn get_last_file_information () -> FantoirFile {
    let endpoint = ExploreApiEndPoint::new(ENDPOINT);
    let result = endpoint.get_dataset_attachments(DATASET_ID).await;

    result
        .attachments
        .into_iter()
        .filter(|attachment| attachment.metas.title.starts_with("Fichier national FANTOIR"))
        .map(|attachment| FantoirFile::from(&attachment).expect("Can't parse FANTOIR file metadata"))
        .max() // The most recent
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggest_fantoir_table () {
        assert_eq!("fantoir_202210", &suggest_fantoir_table("FANTOIR1022"))
    }

    #[test]
    #[should_panic]
    fn test_suggest_fantoir_table_with_bogus_filename () {
        suggest_fantoir_table("FOO");
    }
}
