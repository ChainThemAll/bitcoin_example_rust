use serde::{Deserialize, Serialize};

pub trait Hashable {
    fn hash(&self) -> HashValue;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct HashValue {
    pub hash: [u8; 32],
}
impl std::fmt::Display for HashValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.hash))
    }
}

impl From<[u8; 32]> for HashValue {
    fn from(value: [u8; 32]) -> Self {
        HashValue { hash: value }
    }
}

impl From<String> for HashValue {
    fn from(value: String) -> Self {
        let hash = value.trim_start_matches("0x");
        let bytes = hex::decode(hash).expect("Decoding failed");
        Self {
            hash: bytes.try_into().expect("Conversion failed"),
        }
    }
}
impl HashValue {
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.hash))
    }
}
