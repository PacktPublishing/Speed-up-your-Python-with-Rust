use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct FootPrint {
    pub event_id: i32,
    pub areaperil_id: i32,
    pub intensity_bin_id: i32,
    pub probability: f32
}
