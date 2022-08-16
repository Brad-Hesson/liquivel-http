use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct HttpPostData {
    pub reading: Vec<i16>,
    pub time: std::time::SystemTime,
    pub uuid: u16,
    pub battery_level: f32,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct HttpRespData {
    pub success: bool,
}
