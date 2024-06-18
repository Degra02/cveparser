
#[test]
fn cve_data() {
    let cve_str = std::fs::read_to_string("data/cve_data.json").unwrap();
    let cve: crate::objects::cve_data::CveData = serde_json::from_str(&cve_str).unwrap();

    println!("{:#?}", cve);
}

#[test]
fn impact() {
    let impact_str = std::fs::read_to_string("data/impact.json").unwrap();
    let impact: crate::objects::impact::Impact = serde_json::from_str(&impact_str).unwrap();

    println!("{:#?}", impact);
}

#[test]
fn cve_item() {
    let cve_item_str = std::fs::read_to_string("data/cve.json").unwrap();
    let cve_item: crate::objects::cve_item::CveItem = serde_json::from_str(&cve_item_str).unwrap();

    println!("{:#?}", cve_item);
}

#[test]
fn nvd() {
    let nvd_str = std::fs::read_to_string("data/nvdcve.json").unwrap();
    let nvd: crate::objects::nvd::Nvd = serde_json::from_str(&nvd_str).unwrap();

    println!("{:#?}", nvd);
}
