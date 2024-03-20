use once_cell::sync::Lazy;

use sled::Db;
use std::sync::Mutex;

use crate::{account::Account, block::Block, crypto::Address, hash::HashValue};

pub static DB: Lazy<Mutex<Db>> = Lazy::new(|| {
    let db = sled::open("my_db").expect("failed to open database");
    Mutex::new(db)
});

pub static BITCOIN_PATH: &str = "bitcoin";
pub static UTXO_PATH: &str = "utxo";
pub static WALLET_PATH: &str = "wallet";

// =============================================================================
// blocks
// =============================================================================
pub fn clear() {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    let _ = bitcoin.clear();
}

pub fn add_block(block: Block) {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    let _ = bitcoin.insert(
        block.hash().to_string(),
        serde_json::to_vec(&block).unwrap(),
    );
}
pub fn get_block(hash: HashValue) -> Option<Block> {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    let block = bitcoin.get(hash.to_string()).expect("get block err");
    match block {
        None => None,
        Some(val) => {
            let block: Block = serde_json::from_slice(&val).expect("deserialize block");
            Some(block)
        }
    }
}

pub fn get_height() -> u64 {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    bitcoin.len() as u64
}

// =============================================================================
// transaction
// =============================================================================
pub fn clear_txs() {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    let _ = utxo.clear();
}

pub fn add_transaction(block: Block) {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    let _ = utxo.insert(
        block.hash().to_string(),
        serde_json::to_vec(&block).unwrap(),
    );
}
pub fn get_transaction(hash: HashValue) -> Option<Block> {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    let block = utxo.get(hash.to_string()).expect("get block err");
    match block {
        None => None,
        Some(val) => {
            let block: Block = serde_json::from_slice(&val).expect("deserialize block");
            Some(block)
        }
    }
}
pub fn get_txs_number() -> u64 {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(UTXO_PATH).expect("open tree err");
    bitcoin.len() as u64
}

// =============================================================================
// wallet
// =============================================================================

pub fn add_account(account: Account) {
    let db = DB.lock().expect("db lock err");
    let wallet = db.open_tree(WALLET_PATH).expect("open tree err");
    let _ = wallet.insert(account.address(), serde_json::to_vec(&account).unwrap());
}

pub fn get_all_addresses() -> Vec<Address> {
    let db = DB.lock().expect("db lock err");
    let wallet = db.open_tree(WALLET_PATH).expect("open tree err");

    let mut addresses = Vec::new();
    wallet.iter().for_each(|address| {
        if let Ok((addr, _)) = address {
            addresses.push(String::from_utf8(addr.to_vec()).unwrap());
        }
    });
    addresses
}

#[test]
fn test() {
    let v = "aleo12j4ptnxyj7vwjgrxxusphndxwejaw3cqmdkkshyqmgmxxg40qq9qnqq7gv."
        .to_string()
        .len();

    println!("{}", v);

    let v2 = "APrivateKey1zkpHRjRfDz33wxRM9GuhT8nh64hVJTC4bfL6P4wD6rgFTNQ"
        .to_owned()
        .len();
    println!("{}", v2);
}
