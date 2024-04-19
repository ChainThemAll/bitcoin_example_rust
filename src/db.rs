use once_cell::sync::Lazy;

use core::num;
use sled::Db;
use std::sync::Mutex;

use crate::{
    account::Account,
    block::{Block, BlockHeader},
    crypto::Address,
    hash::HashValue,
    transaction::{TXOutput, Transaction},
};

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
pub fn get_block_last() -> HashValue {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    let (key, value) = bitcoin.last().unwrap().unwrap();
    serde_json::from_slice::<Block>(&value).unwrap().hash()
}

pub fn get_height() -> u64 {
    let db = DB.lock().expect("db lock err");
    let bitcoin = db.open_tree(BITCOIN_PATH).expect("open tree err");
    bitcoin.len() as u64
}

// =============================================================================
// utxo
// =============================================================================
pub fn clear_utxo() {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    let _ = utxo.clear();
}
pub fn add_utxo(addr: Address, tx_out: TXOutput) {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    match utxo.get(addr.clone()) {
        Ok(Some(val)) => {
            let mut outs = serde_json::from_slice::<Vec<TXOutput>>(&val).unwrap();
            outs.push(tx_out);
            let _ = utxo.insert(addr, serde_json::to_vec(&outs).unwrap());
        }
        Ok(None) => {
            let _ = utxo.insert(addr, serde_json::to_vec(&vec![tx_out]).unwrap());
        }
        Err(_) => {
            println!("db_err")
        }
    };
}

pub fn get_utxo(addr: Address) -> Result<Vec<TXOutput>, String> {
    let db = DB.lock().expect("db lock err");
    let utxo = db.open_tree(UTXO_PATH).expect("open tree err");
    match utxo.get(addr) {
        Ok(Some(ivec)) => Ok(serde_json::from_slice::<Vec<TXOutput>>(&ivec).unwrap()),
        Ok(None) => Ok(Vec::new()),
        Err(_) => panic!(),
    }
}

// =============================================================================
// wallet
// =============================================================================

pub fn add_account(account: Account) {
    let db = DB.lock().expect("db lock err");
    let wallet = db.open_tree(WALLET_PATH).expect("open tree err");
    let _ = wallet.insert(account.address(), serde_json::to_vec(&account).unwrap());
}
pub fn get_account(address: Address) -> Result<Account, String> {
    let db = DB.lock().expect("db lock err");
    let wallet = db.open_tree(WALLET_PATH).expect("open tree err");
    match wallet.get(address) {
        Ok(Some(account_data)) => {
            // 尝试反序列化账户数据
            match serde_json::from_slice::<Account>(&account_data) {
                Ok(account) => Ok(account),
                Err(_) => Err("Failed to deserialize the account data.".to_string()),
            }
        }
        Ok(None) => Err("Account not found.".to_string()),
        Err(_) => Err("Database error occurred.".to_string()),
    }
}
pub fn delete_account(account: Account) {
    let db = DB.lock().expect("db lock err");
    let wallet = db.open_tree(WALLET_PATH).expect("open tree err");
    let _ = wallet.remove(account.address());
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

#[test]
fn test_db() {
    let ser = serde_json::to_vec(&Block::new(
        BlockHeader::new(1, HashValue::default(), HashValue::default()),
        &vec![Transaction::default()],
    ))
    .unwrap();

    let r = serde_json::from_slice::<Block>(&ser).unwrap();
    assert_eq!(r.hash(), HashValue::default())
}

#[test]
fn shaung() {
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 3];

    assert_eq!(remove_element(&mut v, 2), 6);

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        for pos in 0..nums.len() {
            if nums[pos] != val {
                nums[left] = nums[pos];
                left += 1;
            }
        }
        left as i32
    }
}
