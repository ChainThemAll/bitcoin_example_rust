use std::time::Duration;

use block_chain_example::{
    block_chain::BlockChain,
    cli::{command, Cli},
    log::log_init,
    transaction::Transaction,
};
use clap::Parser;
use tokio::time::sleep;
use tracing::{info, trace};
static BLOCK_HEIGHT: u32 = 100;
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    command(cli.clone());
    log_init(cli.log.unwrap());
    let block_handle = tokio::spawn(async {
        info!("blockchain start !");

        let mut bitcoin = BlockChain::new();

        for _ in 0..BLOCK_HEIGHT {
            sleep(Duration::from_secs(1)).await;
            bitcoin.add_block(&[Transaction::default()])
        }
    });
    let _ = block_handle.await;
}
