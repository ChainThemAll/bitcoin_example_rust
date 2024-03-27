use serde::{Deserialize, Serialize};
use sha256::{digest, Sha256Digest};

use crate::{
    crypto::{base58_decode, Address, Keypair, PublicKey, ADDRESS_CHECK_SUM_LEN},
    db::{self, get_utxo},
    hash::{HashValue, Hashable},
    signature::{self, Signature},
};

//铸币奖励
const SUBSIDY: u64 = 10;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
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
            vin: vec![TXInput::new(
                HashValue::default(),
                0,
                Signature::default(),
                PublicKey::default(),
            )],
            vout: vec![TXOutput::new(SUBSIDY, to)],
        };

        tx
    }

    pub fn new(from: Address, to: Address, value: u64) -> Self {
        let hash = db::get_block_last();
        let vin = get_utxo(from)
            .unwrap()
            .iter()
            .map(|tx_out| TXInput::new(hash, 0, signature::Signature::default(), [0; 32]))
            .collect::<Vec<TXInput>>();
        let vin = todo!();
        let vout = todo!();
        Self { vin, vout }
    }
    pub fn sign(&mut self, key: &Keypair) {
        self.vin.iter_mut().enumerate().for_each(|(i, vin)| {
            let data = Self::prepare_sign_data();
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
    fn prepare_sign_data() -> String {
        //前一个交易的hash，第几个输出，这次的pubkey,交易输出，自身序列号，签名时间
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
        output.lock(&address);
        output
    }

    fn lock(&mut self, address: &str) {
        let payload = base58_decode(address);

        let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN].to_vec();

        dbg!(pub_key_hash.clone());
        self.pubkey_hash = pub_key_hash.into();
    }
}

#[test]
fn test() {
    let addr = Address::new();
    let co = Transaction::new_coinbase_tx(addr);
    dbg!(co);
}
