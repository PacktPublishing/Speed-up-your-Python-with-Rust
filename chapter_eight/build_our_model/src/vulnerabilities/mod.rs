pub mod structs;
pub mod processes;


use structs::VulnerabilityFootPrint;
use processes::{merge_footprint_with_vulnerabilities, read_vulnerabilities};
use crate::footprint::structs::FootPrint;


pub fn merge_vulnerabilities_with_footprint(footprint: Vec<FootPrint>, mut base_path: String) -> Vec<VulnerabilityFootPrint> {
    let vulnerabilities = read_vulnerabilities(base_path).unwrap();
    return merge_footprint_with_vulnerabilities(vulnerabilities, footprint)
}
