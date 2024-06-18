use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Configurations {
    #[serde(rename = "CVE_data_version")]
    cve_data_version: String,
    #[serde(rename = "nodes")]
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    // operator: String,
    // cpe_match: Vec<CpeMatch>,
}
