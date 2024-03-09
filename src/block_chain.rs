use crate::{block::Block, transaction::Transaction};

#[derive(Debug)]
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
        let r = self.blocks.last_mut().unwrap().get_hash();
        let block = Block::new_block(
            self.blocks.len() as u32,
            self.blocks.last().unwrap().get_hash(),
            transactions,
        );
    }
}
