use block_chain_example::{
    cli::{command, Cli},
    log::log_init,
};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    log_init(cli.log.unwrap());

    command(cli.clone()).await;
}
