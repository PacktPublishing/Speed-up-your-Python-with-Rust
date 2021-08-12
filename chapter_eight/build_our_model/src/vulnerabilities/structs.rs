use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Vulnerability {
    pub vulnerability_id: i32,
    pub intensity_bin_id: i32,
    pub damage_bin_id: i32,
    pub probability: f32
}


#[derive(Debug, Deserialize, Clone)]
pub struct VulnerabilityFootPrint {
    pub vulnerability_id: i32,
    pub intensity_bin_id: i32,
    pub damage_bin_id: i32,
    pub damage_probability: f32,
    pub event_id: i32,
    pub areaperil_id: i32,
    pub footprint_probability: f32,
    pub total_probability: f32
}
