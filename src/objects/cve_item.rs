use super::{configurations::Configurations, cve_data::CveData, impact::Impact};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CveItem {
    pub cve: CveData,
    pub configurations: Configurations,

    pub impact: Impact,

    #[serde(rename = "publishedDate")]
    pub published_date: String,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: String,
}
