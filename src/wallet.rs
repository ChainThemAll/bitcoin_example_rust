use crate::{
    account::{self, Account},
    crypto::Address,
    db,
    transaction::Transaction,
};
//todo!
// manage accounts
// crate new account

#[derive(Debug, Default)]
pub struct Wallet {}

impl Wallet {
    pub fn new() -> Self {
        Self {}
    }
    pub fn add_account(&self, account: Account) {
        db::add_account(account)
    }

    pub fn add_new_account(&self) -> Address {
        let new_account = Account::new();
        Self::add_account(self, new_account.clone());
        new_account.address()
    }
    pub fn get_all_addresses(&self) -> Vec<Address> {
        db::get_all_addresses()
    }
    pub fn send_tx(addr: Address, to: Address, value: u64) {
        let account = db::get_account(addr).unwrap();
        let signed_tx = account.sign_tx(account.gen_tx(to, value));
        println!(
            "Broadcast transactions to the Bitcoin network:{:?}",
            signed_tx
        );
    }
}

#[test]
fn test() {
    let wallet = Wallet::new();
    let addr = wallet.add_new_account();
    dbg!(addr);
    let addr = wallet.add_new_account();
    dbg!(addr);
    let addres = wallet.get_all_addresses();
    dbg!(addres);
}
