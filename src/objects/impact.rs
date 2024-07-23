use serde::{Deserialize, Serialize};

use super::utils::CvssV3Severity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impact {
    #[serde(rename = "baseMetricV3", skip_serializing_if = "Option::is_none")]
    pub base_metric_v3: Option<BaseMetricV3>,

    #[serde(rename = "baseMetricV2", skip_serializing_if = "Option::is_none")]
    pub base_metric_v2: Option<BaseMetricV2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdditionalData {
    #[serde(rename = "impactScore", skip_serializing_if = "Option::is_none")]
    pub impact_score: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(rename = "exploitabilityScore", skip_serializing_if = "Option::is_none")]
    pub exploitability_score: Option<f32>,

    #[serde(rename = "obtainAllPrivilege", skip_serializing_if = "Option::is_none")]
    pub obtain_all_privilege: Option<bool>,

    #[serde(rename = "obtainOtherPrivilege", skip_serializing_if = "Option::is_none")]
    pub obtain_other_privilege: Option<bool>,

    #[serde(rename = "obtainUserPrivilege", skip_serializing_if = "Option::is_none")]
    pub obtain_user_privilege: Option<bool>,

    #[serde(rename = "userInteractionRequired", skip_serializing_if = "Option::is_none")]
    pub user_interaction_required: Option<bool>,

    #[serde(rename = "acInsufInfo", skip_serializing_if = "Option::is_none")]
    pub ac_insuf_info: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMetricV3 {
    #[serde(rename = "cvssV3")]
    pub cvss_v3: CvssV3,

    #[serde(flatten)]
    pub additional_data: AdditionalData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CvssV3 {
    pub version: String,

    #[serde(rename = "vectorString")]
    pub vector_string: String,

    #[serde(rename = "attackVector")]
    pub attack_vector: String,

    #[serde(rename = "attackComplexity")]
    pub attack_complexity: CvssV3Severity,

    #[serde(rename = "privilegesRequired")]
    pub privileges_required: String,

    #[serde(rename = "userInteraction")]
    pub user_interaction: String,

    #[serde(rename = "scope")]
    pub scope: String,

    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: CvssV3Severity,

    #[serde(rename = "integrityImpact")]
    pub integrity_impact: CvssV3Severity,

    #[serde(rename = "availabilityImpact")]
    pub availability_impact: CvssV3Severity,

    #[serde(rename = "baseScore")]
    pub base_score: f32,

    #[serde(rename = "baseSeverity")]
    pub base_severity: CvssV3Severity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMetricV2 {
    #[serde(rename = "cvssV2")]
    pub cvss_v2: CvssV2,

    #[serde(flatten)]
    pub additional_data: AdditionalData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CvssV2 {
    pub version: String,

    #[serde(rename = "vectorString")]
    pub vector_string: String,

    #[serde(rename = "accessVector")]
    pub access_vector: String,

    #[serde(rename = "accessComplexity")]
    pub access_complexity: String,

    #[serde(rename = "authentication")]
    pub authentication: String,

    #[serde(rename = "confidentialityImpact")]
    pub confidentiality_impact: String,

    #[serde(rename = "integrityImpact")]
    pub integrity_impact: String,

    #[serde(rename = "availabilityImpact")]
    pub availability_impact: String,

    #[serde(rename = "baseScore")]
    pub base_score: f32,
}
