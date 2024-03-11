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
        self.blocks.last_mut().unwrap().header.proof_of_work();
        let parent_block_hash = self.blocks.last_mut().unwrap().hash();
        let txs_hash_root = Block::calculate_merkle_root(transactions);
        let block_header =
            BlockHeader::new(self.blocks.len() as u64, parent_block_hash, txs_hash_root);
        self.blocks.push(Block::new(block_header, transactions));
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
