use crypto::*;
use serde::{Deserialize, Serialize};

use crate::crypto::{Address, Keypair, Privatekey, Publickey};
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
}
