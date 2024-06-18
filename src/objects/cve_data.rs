use serde::{Deserialize, Serialize};

use super::utils::{Data, Description, Meta, ProblemType, Reference};


#[derive(Serialize, Deserialize, Debug)]
pub struct CveData {
    #[serde(flatten)]
    data: Data,
    #[serde(rename = "CVE_data_meta")]
    cve_data_meta: Meta,
    problemtype: ProblemType,
    references: Reference,
    description: Description,
}

