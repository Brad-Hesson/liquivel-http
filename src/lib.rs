use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMessage {
    HttpReading {
        reading: Vec<i16>,
        time: std::time::SystemTime,
        uuid: u16,
        battery_level: f32,
    },
    HttpAck,
    Error {
        error: String,
        time: std::time::SystemTime,
    },
}
impl HttpMessage {
    pub fn serialize(&self) -> Vec<u8> {
        // ron::ser::to_string(self).expect("serialization should never fail").into()
        postcard::to_allocvec(self).expect("serialization should never fail")
    }
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        let data = postcard::from_bytes(bytes)?;
        // let data = ron::de::from_bytes(bytes)?;
        Ok(data)
    }
}

#[test]
fn serialization() {
    // let bytes = HttpMessage::Error.serialize();
    let bytes = HttpMessage::HttpReading {
        reading: vec![0, 1, 4, 9],
        time: std::time::SystemTime::now(),
        uuid: 5,
        battery_level: 4.3356,
    }
    .serialize();
    println!("{:?}", bytes);
    println!("{}", String::from_utf8_lossy(&bytes[..]));
}
