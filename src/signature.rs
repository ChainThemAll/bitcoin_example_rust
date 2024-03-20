use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
}

impl Signature {
    pub fn merge(&self) -> [u8; 64] {
        let mut new: [u8; 64] = [0; 64];
        // self.r.join(self.s)
        todo!()
    }
}

impl From<ed25519_dalek::Signature> for Signature {
    fn from(value: ed25519_dalek::Signature) -> Self {
        Self {
            r: value.r_bytes().to_owned(),
            s: value.s_bytes().to_owned(),
        }
    }
}

impl From<Signature> for ed25519_dalek::Signature {
    fn from(value: Signature) -> Self {
        Self::from_components(value.r, value.s)
    }
}
