use serde::{Deserialize, Serialize};

use super::utils::{Data, Description, Meta, ProblemType, Reference};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CveData {
    #[serde(flatten)]
    pub data: Data,
    #[serde(rename = "CVE_data_meta")]
    pub cve_data_meta: Meta,
    pub problemtype: ProblemType,
    pub references: Reference,
    pub description: Description,
}

