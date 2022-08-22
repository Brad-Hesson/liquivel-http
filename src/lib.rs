use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPacket {
    pub time: std::time::SystemTime,
    pub uuid: u32,
    pub message: HttpMessage,
}
impl HttpPacket {
    pub fn new(uuid: u32, message: HttpMessage) -> Self {
        Self {
            time: std::time::SystemTime::now(),
            uuid,
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMessage {
    Reading(Vec<i16>),
    Ack,
    Error(String),
    Status(String),
    SetHibernateTime(u32),
    SetNumReadings(u32),
}
pub fn serialize(messages: &[HttpPacket]) -> Vec<u8> {
    // ron::ser::to_string(self).expect("serialization should never fail").into()
    postcard::to_allocvec(messages).expect("serialization should never fail")
}
pub fn deserialize(bytes: &[u8]) -> Result<Vec<HttpPacket>> {
    // let data = ron::de::from_bytes(bytes)?;
    let data = postcard::from_bytes(bytes)?;
    Ok(data)
}
