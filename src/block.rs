use crate::{hash::HashValue, transaction::Transaction};

use serde::{Deserialize, Serialize};
use sha256::digest;

use std::{
    hash::Hasher,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    parent_block_hash: HashValue,
    hash: HashValue,
    tem_stamp: u64,
    // transactions: Vec<Transaction>,
    data: String,
}

impl Block {
    pub fn new_block(
        height: u32,
        parent_block_hash: HashValue,
        transactions: &[Transaction],
    ) -> Self {
        let mut block = Self {
            index: height,
            parent_block_hash,
            hash: HashValue::default(),
            tem_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: "".to_owned(),
        };
        block.set_hash();
        block
    }

    pub fn gen_genesis_block(transactions: &[Transaction]) -> Self {
        Self::new_block(0, HashValue::default(), transactions)
    }

    pub fn set_hash(&mut self) {
        self.hash = HashValue {
            hash: digest(self.data.clone()).as_bytes().try_into().unwrap(),
        }
    }

    pub fn get_hash(&self) -> HashValue {
        self.hash.clone()
    }

    /*  pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    } */
}

mod tests {
    use crate::{hash::HashValue, transaction::Transaction};

    #[test]
    fn test() {
        use crate::block::Block;
        let block = Block::gen_genesis_block(&[Transaction::default()]);
        dbg!(block);

        let new_block = Block::new_block(1, HashValue::default(), &[Transaction::default()]);
        dbg!(new_block);
    }
}
