use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CvssV3Severity {
    #[serde(rename = "NONE")]
    None,

    #[serde(rename = "LOW")]
    Low,

    #[serde(rename = "MEDIUM")]
    Medium,

    #[serde(rename = "HIGH")]
    High,

    #[serde(rename = "CRITICAL")]
    Critical,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    data_type: String,
    data_format: String,
    data_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    reference_data: Vec<ReferenceData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceData {
    url: String,
    name: String,
    refsource: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProblemType {
    problemtype_data: Vec<ProblemTypeData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProblemTypeData {
    description: Vec<DescriptionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    description_data: Vec<DescriptionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionData {
    lang: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "ASSIGNER")]
    assigner: String,
}

