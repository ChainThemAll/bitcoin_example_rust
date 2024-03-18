use crate::account::Account;
//todo!
// manage accounts
// crate new account

#[derive(Debug, Default)]
pub struct Wallet {
    accounts: Vec<Account>,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account)
    }

    pub fn add_new_account(&mut self) {
        let new_account = Account::new();
        Self::add_account(self, new_account);
    }
}

#[test]
fn test() {
    let mut wallet = Wallet::new();
    wallet.add_new_account();
    dbg!(wallet);
}
