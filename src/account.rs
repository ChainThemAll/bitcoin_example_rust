use crate::{
    crypto::{ripemd160_digest, Address, Keypair, PrivateKey, PublicKey, Ripemd160Hash},
    hash::HashValue,
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
        let Keypair: Keypair = Keypair::new();

        let private_key = Keypair.private_key();

        let pubkey = Keypair.public_key();

        let address = Keypair.address();
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
}

#[test]
fn test() {
    let a = Account::new();
    println!("{}", a.address())
}
