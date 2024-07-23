use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configurations {
    #[serde(rename = "CVE_data_version")]
    pub cve_data_version: String,
    #[serde(rename = "nodes")]
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    // operator: String,
    // cpe_match: Vec<CpeMatch>,
}
