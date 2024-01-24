
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wrapper {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}