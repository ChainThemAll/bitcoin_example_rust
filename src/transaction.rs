use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::{
    adddress::Address,
    hash::{HashValue, Hashable},
    signature::Signature,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXInput {
    previous_output_hash: HashValue,
    script_sig: Signature,
}

impl TXInput {
    pub fn new(previous_output_hash: HashValue, script_sig: Signature) -> Self {
        Self {
            previous_output_hash,
            script_sig,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXOutput {
    value: u64,
    script_pubkey: Address,
}

impl TXOutput {
    pub fn new(value: u64, script_pubkey: Address) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }
}

impl Transaction {
    pub fn new(vin: Vec<TXInput>, vout: Vec<TXOutput>) -> Self {
        Self { vin, vout }
    }

    pub fn coinbase(vout: Vec<TXOutput>) -> Self {
        Self {
            vin: vec![TXInput::new(HashValue::default(), Signature::default())],
            vout,
        }
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> HashValue {
        let ser_tx: Vec<u8> = serde_json::to_vec(&self).unwrap();
        let hash = digest(ser_tx);
        HashValue::from(hash)
    }
}
