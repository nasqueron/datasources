//! Integration tests for API requests

use std::collections::HashMap;

use lazy_static::lazy_static;
use mockito::{mock, Mock};
use serde_json::json;

use opendatasoft_explore_api::requests::ExploreApiEndPoint;
use opendatasoft_explore_api::schema::*;

static TEST_URL: &'static str = "https://data.economie.gouv.fr/api/v2";
static TEST_DATASET_ID: &'static str = "fichier-fantoir-des-voies-et-lieux-dits";

static TEST_DATASET_WITH_RECORDS_ID: &'static str = "controle_techn";
static TEST_RECORD_ID: &'static str = "eb04cba18e872814448a7fda829f3f1918cfae0b";

lazy_static! {
    static ref MOCK_URL: String = mockito::server_url();

    static ref MOCK_FILES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("/catalog/datasets", include_str!("requests/catalog_datasets.json"));
        m.insert("/catalog/facets", include_str!("requests/catalog_facets.json"));
        m.insert("/catalog/exports/rdf", include_str!("requests/catalog_exports.rdf"));
        m.insert("/catalog/datasets/controle_techn/records", include_str!("requests/catalog_datasets_records.json"));
        m.insert("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits", include_str!("requests/catalog_dataset_fantoir.json"));
        m.insert("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits/attachments", include_str!("requests/catalog_dataset_fantoir_attachments.json"));
        m.insert("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits/facets", include_str!("requests/catalog_dataset_fantoir_facets.json"));
        m.insert("/catalog/datasets/controle_techn/records/eb04cba18e872814448a7fda829f3f1918cfae0b", include_str!("requests/catalog_dataset_record.json"));
        m
    };
}

pub fn prepare_mock (url: &str) -> Mock {
    mock("GET", url)
        .with_status(200)
        .with_body(MOCK_FILES[url])
        .create()
}

#[tokio::test]
async fn test_get_datasets () {
    let _mock = prepare_mock("/catalog/datasets");

    let endpoint = ExploreApiEndPoint::new(&MOCK_URL);
    let catalog = endpoint.get_datasets().await;

    assert_eq!(426, catalog.total_count);
    assert_eq!(Link {
        href: "https://data.economie.gouv.fr/api/v2/catalog/datasets/mef-catalogue-temporaire".to_string(),
        rel: "self".to_string(),
    }, catalog.datasets[0].links[0]);
    assert_eq!(3, catalog.datasets.len());
}

#[tokio::test]
async fn test_export_datasets_catalog () {
    let _mock = prepare_mock("/catalog/exports/rdf");

    let mut response = ExploreApiEndPoint::new(&MOCK_URL)
        .export_datasets_catalog("rdf")
        .await;

    let mut rdf_about_found = false;
    while let Some(chunk) = response.chunk().await.unwrap() {
        let part = String::from_utf8(chunk.to_vec()).unwrap();
        if part.contains("rdf:about") {
            rdf_about_found = true;
            break;
        }
    }

    assert!(rdf_about_found);
}

#[tokio::test]
async fn test_get_facets () {
    let _mock = prepare_mock("/catalog/facets");

    let endpoint = ExploreApiEndPoint::new(&MOCK_URL);
    let facets = endpoint.get_facets().await;

    assert!(facets.links[0].href.starts_with(TEST_URL));

    let expected_facets_categories = vec![
        "features".to_string(), "modified".to_string(),
        "publisher".to_string(), "keyword".to_string(),
        "theme".to_string(),
    ];
    let actual_facets_categories: Vec<_> = facets.facets
        .into_iter()
        .map(|facet| facet.name)
        .collect();

    assert_eq!(expected_facets_categories, actual_facets_categories);
}

#[tokio::test]
async fn test_get_dataset_records () {
    let _mock = prepare_mock("/catalog/datasets/controle_techn/records");

    let results = ExploreApiEndPoint::new(&MOCK_URL)
        .get_dataset_records(TEST_DATASET_WITH_RECORDS_ID)
        .await;

    assert_eq!(222629, results.total_count);

    let record = match &results.records[0] {
        ResultsRecord::Aggregation(_) => unreachable!(),
        ResultsRecord::Record(record) => record.clone(),
    };
    assert_eq!("b839362b229db63bc9b344e980ae6273be7f80fd", record.record.id.as_str());
    assert_eq!(
        Some(&json!("Voiture Particulière")),
        record.record.fields.get("cat_vehicule_libelle")
    );

    let link = &record.links[0];
    assert!(link.href.starts_with(TEST_URL));
    assert!(link.href.contains(TEST_DATASET_WITH_RECORDS_ID));
}

#[tokio::test]
async fn test_get_dataset_information () {
    let _mock = prepare_mock("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits");

    let dataset = ExploreApiEndPoint::new(&MOCK_URL)
        .get_dataset_information(TEST_DATASET_ID)
        .await;

    assert_eq!(TEST_DATASET_ID, dataset.dataset.dataset_id);
}

#[tokio::test]
async fn test_get_dataset_attachments () {
    let _mock = prepare_mock("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits/attachments");

    let attachments = ExploreApiEndPoint::new(&MOCK_URL)
        .get_dataset_attachments(TEST_DATASET_ID)
        .await;

    assert!(attachments.attachments[0].metas.url.starts_with("odsfile://"));
}

#[tokio::test]
async fn test_get_dataset_facets () {
    let _mock = prepare_mock("/catalog/datasets/fichier-fantoir-des-voies-et-lieux-dits/facets");

    let facets = ExploreApiEndPoint::new(&MOCK_URL)
        .get_dataset_facets(TEST_DATASET_ID)
        .await;

    assert!(facets.links[0].href.starts_with(TEST_URL));
}

#[tokio::test]
async fn test_get_dataset_record () {
    let _mock = prepare_mock("/catalog/datasets/controle_techn/records/eb04cba18e872814448a7fda829f3f1918cfae0b");

    let record = ExploreApiEndPoint::new(&MOCK_URL)
        .get_dataset_record(TEST_DATASET_WITH_RECORDS_ID, TEST_RECORD_ID)
        .await;

    assert_eq!(TEST_RECORD_ID, record.record.id);
}
