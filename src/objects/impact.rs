use serde::{Deserialize, Serialize};

use super::utils::CvssV3Severity;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Impact {
    #[serde(rename = "baseMetricV3", skip_serializing_if = "Option::is_none")]
    base_metric_v3: Option<BaseMetricV3>,

    #[serde(rename = "baseMetricV2", skip_serializing_if = "Option::is_none")]
    base_metric_v2: Option<BaseMetricV2>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdditionalData {
    #[serde(rename = "impactScore", skip_serializing_if = "Option::is_none")]
    impact_score: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<String>,

    #[serde(rename = "exploitabilityScore", skip_serializing_if = "Option::is_none")]
    exploitability_score: Option<f32>,

    #[serde(rename = "obtainAllPrivilege", skip_serializing_if = "Option::is_none")]
    obtain_all_privilege: Option<bool>,

    #[serde(rename = "obtainOtherPrivilege", skip_serializing_if = "Option::is_none")]
    obtain_other_privilege: Option<bool>,

    #[serde(rename = "obtainUserPrivilege", skip_serializing_if = "Option::is_none")]
    obtain_user_privilege: Option<bool>,

    #[serde(rename = "userInteractionRequired", skip_serializing_if = "Option::is_none")]
    user_interaction_required: Option<bool>,

    #[serde(rename = "acInsufInfo", skip_serializing_if = "Option::is_none")]
    ac_insuf_info: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMetricV3 {
    #[serde(rename = "cvssV3")]
    cvss_v3: CvssV3,

    #[serde(flatten)]
    additional_data: AdditionalData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CvssV3 {
    version: String,

    #[serde(rename = "vectorString")]
    vector_string: String,

    #[serde(rename = "attackVector")]
    attack_vector: String,

    #[serde(rename = "attackComplexity")]
    attack_complexity: CvssV3Severity,

    #[serde(rename = "privilegesRequired")]
    privileges_required: String,

    #[serde(rename = "userInteraction")]
    user_interaction: String,

    #[serde(rename = "scope")]
    scope: String,

    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: CvssV3Severity,

    #[serde(rename = "integrityImpact")]
    integrity_impact: CvssV3Severity,

    #[serde(rename = "availabilityImpact")]
    availability_impact: CvssV3Severity,

    #[serde(rename = "baseScore")]
    base_score: f32,

    #[serde(rename = "baseSeverity")]
    base_severity: CvssV3Severity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMetricV2 {
    #[serde(rename = "cvssV2")]
    cvss_v2: CvssV2,

    #[serde(flatten)]
    additional_data: AdditionalData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CvssV2 {
    version: String,

    #[serde(rename = "vectorString")]
    vector_string: String,

    #[serde(rename = "accessVector")]
    access_vector: String,

    #[serde(rename = "accessComplexity")]
    access_complexity: String,

    #[serde(rename = "authentication")]
    authentication: String,

    #[serde(rename = "confidentialityImpact")]
    confidentiality_impact: String,

    #[serde(rename = "integrityImpact")]
    integrity_impact: String,

    #[serde(rename = "availabilityImpact")]
    availability_impact: String,

    #[serde(rename = "baseScore")]
    base_score: f32,
}
