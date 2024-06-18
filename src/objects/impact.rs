use serde::{Deserialize, Serialize};

use super::utils::CvssV3Severity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Impact {
    #[serde(rename = "baseMetricV3", skip_serializing_if = "Option::is_none")]
    base_metric_v3: Option<BaseMetricV3>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMetricV3 {
    #[serde(rename = "cvssV3")]
    cvss_v3: CvssV3,
}

#[derive(Serialize, Deserialize, Debug)]
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
