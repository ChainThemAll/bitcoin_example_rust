use crate::crypto::{Address, Keypair, Privatekey, Publickey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Account {
    private_key: Privatekey,
    pubkey: Publickey,
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

    pub fn prikey(&self) -> Privatekey {
        self.prikey()
    }
    pub fn pubkey(&self) -> Publickey {
        self.pubkey()
    }
    pub fn address(&self) -> Address {
        self.address()
    }
}
