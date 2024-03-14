use tracing::trace;

use crate::{
    block::{Block, BlockHeader},
    transaction::Transaction,
};

#[derive(Debug, Default)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::gen_genesis_block(&[Transaction::default()])],
        }
    }
    pub fn add_block(&mut self, transactions: &[Transaction]) {
        trace!("|--------------------------------block----------------------------------|");
        self.blocks.last_mut().unwrap().header.proof_of_work();
        trace!("| height: {}", self.blocks.len());
        let parent_block_hash = self.blocks.last_mut().unwrap().hash();
        let txs_hash_root = Block::calculate_merkle_root(transactions);
        trace!("| hash: {}", txs_hash_root);
        let block_header = BlockHeader::new(
            self.blocks.len() as u64,
            parent_block_hash,
            txs_hash_root.clone(),
        );
        self.blocks.push(Block::new(block_header, transactions));
        trace!("|-----------------------------------------------------------------------|");
        trace!("                                  ||                                     ");
        trace!("                                  \\/                                    ");
    }

    pub fn show(&self) {
        for block in self.blocks.iter() {
            println!("|--------------------------------block----------------------------------|");
            println!("| height: {}", block.height());
            println!("| hash: {}", block.hash());
            println!("|-----------------------------------------------------------------------|");
            println!("                                  ||                                     ");
            println!("                                  \\/                                    ");
        }
    }
}

#[test]
fn test() {}
