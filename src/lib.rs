use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpPacket {
    pub time: std::time::SystemTime,
    pub dev_id: u64,
    pub message: HttpMessage,
}
impl HttpPacket {
    pub fn new(dev_id: u64, message: HttpMessage) -> Self {
        Self {
            time: std::time::SystemTime::now(),
            dev_id,
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
    FirmwareVersion([u8; 32]),
    Update,
    ElfRequest(u32),
    ElfData(Vec<u8>),
    ElfDone,
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
