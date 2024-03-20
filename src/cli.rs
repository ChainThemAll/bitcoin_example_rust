use std::time::Duration;

use clap::{Parser, Subcommand};
use tokio::time::sleep;
use tracing::info;

use crate::{
    block_chain::BlockChain,
    transaction::Transaction,
    wallet::{self, Wallet},
};
static BLOCK_HEIGHT: u32 = 100;

/// Manages a custom Rust implementation of a Bitcoin blockchain
#[derive(Parser, Clone)]
#[command(version = "1.0", author = "alain bohn", about = "A Rust Bitcoin Blockchain CLI", long_about = None)]
pub struct Cli {
    /// Runs the client as a server to accept JSON-RPC commands
    #[arg(long)]
    pub server: bool,

    /// Sets the username for RPC connections
    #[arg(long)]
    pub rpcuser: Option<String>,

    /// Sets the password for RPC connections
    #[arg(long)]
    pub rpcpassword: Option<String>,

    /// Indicates the client to use the test network
    #[arg(long)]
    pub testnet: bool,

    /// Sets the directory for block data storage
    #[arg(long)]
    pub datadir: Option<String>,

    /// Specifies the path to the configuration file
    #[arg(long)]
    pub conf: Option<String>,

    #[arg(short, default_value = "trace")]
    pub log: Option<tracing::Level>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Returns the status information of the blockchain
    Getblockchaininfo,

    /// Returns detailed information of a specified block
    Getblock {
        /// The hash of the block
        block_hash: String,
    },

    /// Queries detailed information of a specific transaction
    Gettransaction {
        /// The ID of the transaction
        transaction_id: String,
    },

    /// Sends a specified amount of Bitcoin to an address
    Sendtoaddress {
        /// The Bitcoin address
        address: String,
        /// The amount of Bitcoin to send
        amount: f64,
    },

    /// Creates a new address for receiving Bitcoin
    Getnewaddress,

    /// Returns the total balance of the wallet
    Getbalance,
}

pub async fn command(cli: Cli) {
    println!("im in");
    match &cli.command {
        Some(Commands::Getblockchaininfo) => {
            println!("Getting blockchain info...");
            // Implement the logic of obtaining blockchain information
        }

        Some(Commands::Getblock { block_hash }) => {
            println!("Getting info for block: {}", block_hash);
            // Implement the logic to obtain specified block information
        }

        Some(Commands::Gettransaction { transaction_id }) => {
            println!("Getting transaction info: {}", transaction_id);
            // Implement the logic to obtain specific transaction information
        }

        Some(Commands::Sendtoaddress { address, amount }) => {
            println!("Sending {} to address: {}", amount, address);
        }

        Some(Commands::Getnewaddress) => {
            info!("Creating a new address...");
            //Implement the logic for creating new addresses
            let address = Wallet::new().add_new_account();
            info!("This is your enw account address : '{}'", address)
        }

        Some(Commands::Getbalance) => {
            println!("Getting wallet balance...");
            // Implement the logic of obtaining wallet balance
        }

        None => {
            println!("Running as server...");
            let block_handle = tokio::spawn(async {
                let mut bitcoin = BlockChain::new();

                for _ in 0..BLOCK_HEIGHT {
                    sleep(Duration::from_secs(1)).await;
                    bitcoin.add_block(&[Transaction::default()])
                }
            });
            let _ = block_handle.await;
        }
    }
}
