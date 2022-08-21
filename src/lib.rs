use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMessage {
    Reading {
        reading: Vec<i16>,
        time: std::time::SystemTime,
    },
    Ack,
    Error {
        error: String,
        time: std::time::SystemTime,
    },
    Status {
        status: String,
        time: std::time::SystemTime,
    },
    SetHibernateTime{
        ms: u32
    },
    SetNumReadings{
        num: u32
    }
}
pub fn serialize(messages: &[HttpMessage]) -> Vec<u8> {
    // ron::ser::to_string(self).expect("serialization should never fail").into()
    postcard::to_allocvec(messages).expect("serialization should never fail")
}
pub fn deserialize(bytes: &[u8]) -> Result<Vec<HttpMessage>> {
    // let data = ron::de::from_bytes(bytes)?;
    let data = postcard::from_bytes(bytes)?;
    Ok(data)
}
