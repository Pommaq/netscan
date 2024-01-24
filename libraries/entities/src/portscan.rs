pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub addr: String,
    pub ports: Vec<u16>,
}
impl Address {
    pub fn new(addr: &str, ports: &[u16]) -> Self {
        Self {
            addr: addr.to_string(),
            ports: ports.to_vec(),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Port {
    pub port: u16,
}

impl Port {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[cfg(test)]
mod tests {

    use super::{Address, Port};
    #[test]
    fn serialize_ip() {
        let target = Address::new("1.1.1.1", &[1337, 80, 443]);
        let ser = bincode::serialize(&target).unwrap();
        assert_eq!(
            ser,
            [
                7, 0, 0, 0, 0, 0, 0, 0, 49, 46, 49, 46, 49, 46, 49, 3, 0, 0, 0, 0, 0, 0, 0, 57, 5,
                80, 0, 187, 1
            ]
        );
    }
    #[test]
    /// Mostly here to document how we do this
    fn test_bincode() {
        let port = Port { port: 1337 };

        let ser = bincode::serialize(&port).unwrap();

        assert_eq!(ser, vec![57, 5]);
    }
}
