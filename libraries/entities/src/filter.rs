#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wrapper {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}
impl Wrapper {
    pub fn new(key: &[u8], value: &[u8]) -> Self {
        Self {
            key: key.to_vec(),
            value: value.to_vec(),
        }
    }
}
/// Remove the wrapper from the byte input. Useful for subscriptions where we dont care about the key
pub fn unwrap(bytes: &[u8]) -> Result<Vec<u8>, crate::Error> {
    let decoded: Wrapper = bincode::deserialize(bytes)?;
    Ok(decoded.value)
}

#[cfg(test)]
mod tests {
    use crate::filter::{unwrap, Wrapper};

    #[test]
    fn test_wrapper() {
        const PAYLOAD: &[u8] = b"aaaaa";
        let wrap = Wrapper::new(b"BOGUS", PAYLOAD);
        let serialized = bincode::serialize(&wrap).unwrap();
        let deserialized: Vec<u8> = unwrap(&serialized).unwrap();
        assert_eq!(PAYLOAD, deserialized);
    }
}
