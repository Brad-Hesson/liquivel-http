use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct HttpPostData {
    pub reading: Vec<i16>,
    pub time: std::time::SystemTime,
    pub uuid: u16,
    pub battery_level: f32,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct HttpRespData {
    pub success: bool,
}

pub trait SerializeBytes
where
    Self: Serialize,
{
    fn as_bytes(&self) -> Option<Vec<u8>> {
        ron::ser::to_string(self).ok().map(String::into)
    }
}
impl<T> SerializeBytes for T where T: Serialize {}
pub trait DeserializeBytes<'a>
where
    Self: Deserialize<'a>,
{
    fn from_bytes(bytes: &'a [u8]) -> Option<Self> {
        ron::de::from_bytes(bytes).ok()
    }
}
impl<'de, T> DeserializeBytes<'de> for T where T: Deserialize<'de> {}
