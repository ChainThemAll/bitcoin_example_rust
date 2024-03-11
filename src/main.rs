use block_chain_example::{
    block_chain::BlockChain,
    transaction::{TXInput, TXOutput, Transaction},
};

fn main() {
    let mut block_chain = BlockChain::new();
    block_chain.add_block(&[Transaction::new(
        vec![1, 2, 3],
        vec![TXInput("7".to_owned())],
        vec![TXOutput("5".to_owned())],
    )]);
    block_chain.add_block(&[Transaction::new(
        vec![1, 2, 3],
        vec![TXInput("3".to_owned())],
        vec![TXOutput("2".to_owned())],
    )]);
    dbg!(block_chain);
}
