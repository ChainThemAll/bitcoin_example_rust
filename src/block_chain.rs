use tracing::trace;

use crate::{
    block::{Block, BlockHeader},
    db,
    hash::HashValue,
    transaction::Transaction,
};

#[derive(Debug, Default)]
pub struct BlockChain {
    last_hash: HashValue,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut block = Block::gen_genesis_block(&[Transaction::default()]);
        block.header.proof_of_work();
        let hash = block.hash();
        db::add_block(block);
        Self { last_hash: hash }
    }
    pub fn add_block(&mut self, transactions: &[Transaction]) {
        trace!("|--------------------------------block----------------------------------|");

        trace!("| height: {}", db::get_height());
        let parent_block_hash = self.last_hash;
        trace!("| parent_block_hash: {}", parent_block_hash);
        let txs_hash_root = Block::calculate_merkle_root(transactions);
        trace!("| txs_hash_root: {}", txs_hash_root);
        let block_header = BlockHeader::new(db::get_height(), parent_block_hash, txs_hash_root);
        let mut block = Block::new(block_header, transactions);
        block.header.proof_of_work();
        trace!("| hash: {}", block.hash());
        self.last_hash = block.hash();
        db::add_block(block);
        trace!("|-----------------------------------------------------------------------|");
        trace!("                                  ||                                     ");
        trace!("                                  \\/                                    ");
    }
}
