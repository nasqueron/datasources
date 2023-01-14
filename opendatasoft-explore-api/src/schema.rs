//! Schema for Opendatasoft Explore API v2

use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

/*  -------------------------------------------------------------
    links
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

/*  -------------------------------------------------------------
    dataset
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dataset {
    pub links: Vec<Link>,
    pub dataset: DatasetProperties,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetProperties {
    pub dataset_id: String,
    pub dataset_uid: String,
    pub attachments: Vec<AttachmentProperties>,
    pub has_records: bool,
    pub data_visible: bool,
    /// A map of available features for a dataset, with the fields they apply to.
    pub features: Vec<String>,
    pub metas: JsonValue,
    pub fields: Vec<DatasetField>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<JsonValue>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetField {
    pub name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub annotations: JsonValue,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/*  -------------------------------------------------------------
    results_dataset
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetsCollection {
    pub total_count: usize,
    pub links: Vec<Link>,
    pub datasets: Vec<Dataset>,
}

/*  -------------------------------------------------------------
    facet_value_enumeration
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FacetValueEnumeration {
    pub name: String,
    pub count: usize,
    pub value: String,
    pub state: String,
}

/*  -------------------------------------------------------------
    facet_enumeration
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FacetEnumeration {
    pub name: String,
    pub facets: Vec<FacetValueEnumeration>,
}

/*  -------------------------------------------------------------
    aggregation
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Aggregation {
    pub count: usize,
    pub cou_name_en: String,
}

/*  -------------------------------------------------------------
    record
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Record {
    pub record: RecordProperties,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecordProperties {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub size: usize,
    pub fields: JsonValue,
}

/*  -------------------------------------------------------------
    results
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Results {
    pub total_count: usize,
    pub links: Vec<Link>,
    pub records: Vec<ResultsRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultsRecord {
    Aggregation(Aggregation),
    Record(Record),
}

/*  -------------------------------------------------------------
    attachment
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Attachment {
    pub href: String,
    pub metas: AttachmentProperties,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachmentProperties {
    #[serde(rename = "mime-type", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    pub title: String,
    pub url: String,
    pub id: String,
}

/*  -------------------------------------------------------------
    Response to /catalog/datasets/{dataset_id}/attachments
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachmentCollection {
    pub links: Vec<Link>,
    pub attachments: Vec<Attachment>,
}

/*  -------------------------------------------------------------
    Response to /catalog/facets
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FacetsCollection {
    pub links: Vec<Link>,
    pub facets: Vec<FacetEnumeration>,
}
