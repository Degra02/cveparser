use super::{configurations::Configurations, cve_data::CveData, impact::Impact};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CveItem {
    cve: CveData,
    configurations: Configurations,

    impact: Impact,

    #[serde(rename = "publishedDate")]
    published_date: String,

    #[serde(rename = "lastModifiedDate")]
    last_modified_date: String,
}
