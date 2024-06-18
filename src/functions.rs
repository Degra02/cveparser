
pub fn parse_all_cves() -> Vec<crate::objects::cve_item::CveItem> {
    let nvd_file = std::env::var("NVD_FILE").unwrap_or("data/nvdcve.json".to_string());
    let nvd_str = std::fs::read_to_string(nvd_file).unwrap();
    let nvd: crate::objects::nvd::Nvd = serde_json::from_str(&nvd_str).unwrap();

    nvd.cve_items
}
