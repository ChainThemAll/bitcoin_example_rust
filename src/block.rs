use crate::{
    hash::{HashValue, Hashable},
    transaction::Transaction,
};

use serde::{Deserialize, Serialize};
use sha256::digest;

use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
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
    pub fn hash(&self) -> HashValue {
        self.header.hash()
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    height: u32,
    parent_block_hash: HashValue,
    tem_stamp: u64,
    merkle_root: HashValue,
}
impl Hashable for BlockHeader {
    fn hash(&self) -> HashValue {
        let serialized_block_header = serde_json::to_vec(&self).unwrap();
        let hash = digest(serialized_block_header);
        HashValue::from(hash)
    }
}

impl BlockHeader {
    pub fn new(height: u32, parent_block_hash: HashValue, merkle_root: HashValue) -> Self {
        Self {
            height,
            parent_block_hash,
            tem_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            merkle_root,
        }
    }
}

#[test]
fn test() {
    let header = BlockHeader::new(1, HashValue::default(), HashValue::default());
    let hash = header.hash();
    println!("{}", hash);
}
