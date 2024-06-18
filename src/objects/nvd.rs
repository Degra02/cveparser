use serde::{Deserialize, Serialize};

use super::cve_item::CveItem;

#[derive(Serialize, Deserialize, Debug)]
pub struct Nvd {
    #[serde(rename = "CVE_data_type")]
    pub cve_data_type: String,

    #[serde(rename = "CVE_data_format")]
    pub cve_data_format: String,

    #[serde(rename = "CVE_data_version")]
    pub cve_data_version: String,

    #[serde(rename = "CVE_data_numberOfCVEs")]
    pub cve_data_number_of_cves: String,
    
    #[serde(rename = "CVE_data_timestamp")]
    pub cve_data_timestamp: String,

    #[serde(rename = "CVE_Items")]
    pub cve_items: Vec<CveItem>,
}
