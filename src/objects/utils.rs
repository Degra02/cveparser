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
    pub data_type: String,
    pub data_format: String,
    pub data_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    pub reference_data: Vec<ReferenceData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceData {
    pub url: String,
    pub name: String,
    pub refsource: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProblemType {
    pub problemtype_data: Vec<ProblemTypeData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProblemTypeData {
    pub description: Vec<DescriptionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub description_data: Vec<DescriptionData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DescriptionData {
    pub lang: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ASSIGNER")]
    pub assigner: String,
}

