use crate::{
    hash::{HashValue, Hashable},
    transaction::Transaction,
};

use serde::{Deserialize, Serialize};
use sha256::digest;

use std::time::{SystemTime, UNIX_EPOCH};

const BITS: u32 = 2;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    height: u64,
    parent_block_hash: HashValue,
    time_stamp: u64,
    merkle_root: HashValue,
    bits: u32,
    nonce: u64,
}
impl Block {
    pub fn new(header: BlockHeader, txs: &[Transaction]) -> Self {
        Self {
            header,
            transactions: txs.to_vec(),
        }
    }
    pub fn gen_genesis_block(txs: &[Transaction]) -> Self {
        let header = BlockHeader::new(0, HashValue::default(), HashValue::default());
        Self::new(header, txs)
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> HashValue {
        let mut layer = transactions.iter().map(|tx| tx.hash()).collect::<Vec<_>>();

        while layer.len() > 1 {
            let mut new_layer = Vec::new();
            for chunk in layer.chunks(2) {
                let hash = if chunk.len() == 2 {
                    let v = vec![chunk[0], chunk[1]];
                    let hashs = serde_json::to_vec(&v).unwrap();
                    let hash = digest(hashs);
                    HashValue::from(hash)
                } else {
                    chunk[0]
                };
                new_layer.push(hash);
            }
            layer = new_layer;
        }

        layer[0]
    }
    pub fn height(&self) -> u64 {
        self.header.height
    }
    pub fn hash(&self) -> HashValue {
        self.header.hash()
    }
}
impl Hashable for BlockHeader {
    fn hash(&self) -> HashValue {
        let serialized_block_header = serde_json::to_vec(&self).unwrap();
        let hash = digest(serialized_block_header);
        HashValue::from(hash)
    }
}

impl BlockHeader {
    pub fn new(height: u64, parent_block_hash: HashValue, merkle_root: HashValue) -> Self {
        Self {
            height,
            parent_block_hash,
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            merkle_root,
            bits: BITS,
            nonce: 0,
        }
    }
    fn calculate_target(&self) -> HashValue {
        let bits = self.bits;
        let mut output = "f".repeat(64);
        let input = bits as usize;

        if input < 64 {
            for i in 0..input {
                output.replace_range(i..i + 1, "0");
            }
            output.replace_range(input..input + 1, "f");
        }

        HashValue::from(output)
    }
    pub fn proof_of_work(&mut self) {
        let target = self.calculate_target();

        while self.nonce < u64::MAX {
            let hash = self.hash();
            if hash < target {
                break;
            } else {
                self.nonce += 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut header = BlockHeader::new(1, HashValue::default(), HashValue::default());
    let target = header.calculate_target();
    println!("Target: {:?}", target.to_string());
    header.proof_of_work();
    println!("{:?}", header.nonce);
    println!("{:?}", header.hash());
}
