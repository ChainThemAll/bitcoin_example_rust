use crate::{
    crypto::{ripemd160_digest, Address, Keypair, PrivateKey, PublicKey, Ripemd160Hash},
    hash::HashValue,
    transaction::Transaction,
};
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Account {
    private_key: PrivateKey,
    pubkey: PublicKey,
    address: Address,
}

impl Account {
    pub fn new() -> Self {
        let keypair: Keypair = Keypair::new();

        let private_key = keypair.private_key();

        let pubkey = keypair.public_key();

        let address = keypair.address();
        Self {
            private_key,
            pubkey,
            address,
        }
    }

    pub fn private_key(&self) -> PrivateKey {
        self.private_key
    }
    pub fn public_key(&self) -> PublicKey {
        self.pubkey
    }
    pub fn address(&self) -> Address {
        self.address.clone()
    }
    pub fn pub_key_hash(&self) -> Ripemd160Hash {
        ripemd160_digest(self.pubkey.digest().as_bytes())
    }
    pub fn gen_tx(&self, to: Address, value: u64) -> Transaction {
        Transaction::new(self.address.clone(), to, value)
    }
    pub fn sign_tx(&self, mut tx: Transaction) -> Transaction {
        let keypair: Keypair = Keypair::from_bytes(&self.private_key);
        tx.sign(&keypair);
        tx
    }
}

#[test]
fn test() {
    let a = Account::new();
    println!("{}", a.address())
}
