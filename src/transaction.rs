use serde::{Deserialize, Serialize};
use sha256::{digest, Sha256Digest};

use crate::{
    crypto::{Address, Keypair, PublicKey},
    hash::{HashValue, Hashable},
    signature::Signature,
};

const SUBSIDY: u64 = 10;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: HashValue,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXInput {
    //前一个交易哈希
    previous_output_hash: HashValue,
    // 前一个交易中的第几个输出
    vout: u32,
    //签名信息
    signature: Signature,
    //签名者的公钥，用来让别人验证
    pubkey: PublicKey,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXOutput {
    value: u64,
    pubkey_hash: HashValue,
}

impl Transaction {
    pub fn new_coinbase_tx(to: Address) -> Self {
        let mut tx = Self {
            id: HashValue::default(),
            vin: vec![TXInput::new(
                HashValue::default(),
                0,
                Signature::default(),
                PublicKey::default(),
            )],
            vout: vec![TXOutput::new(SUBSIDY, to)],
        };
        tx.id = tx.hash();
        tx
    }
    pub fn sign(&mut self, key: &Keypair) {
        self.vin.iter_mut().enumerate().for_each(|(i, vin)| {
            let data = self.id.to_string();
            let data_hash = data.digest();
            let signature = Signature::sign(key, &data_hash.into());
            vin.previous_output_hash = HashValue::default();
            vin.pubkey = key.public_key();
            vin.vout = i as u32;
            vin.signature = signature;
        });
    }
    pub fn verify() -> bool {
        todo!()
    }
}

impl Hashable for Transaction {
    fn hash(&self) -> HashValue {
        let ser_tx: Vec<u8> = serde_json::to_vec(&self).unwrap();
        let hash = digest(ser_tx);
        HashValue::from(hash)
    }
}

impl TXInput {
    pub fn new(
        previous_output_hash: HashValue,
        vout: u32,
        signature: Signature,
        pubkey: PublicKey,
    ) -> Self {
        Self {
            previous_output_hash,
            vout,
            signature,
            pubkey,
        }
    }
}

impl TXOutput {
    pub fn new(value: u64, address: Address) -> Self {
        let mut output = TXOutput {
            value,
            pubkey_hash: HashValue::default(),
        };
        output.lock(address);
        output
    }

    fn lock(&self, address: String) {
        todo!()
    }
}
