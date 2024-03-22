use crate::signature::Signature;
use crypto::{digest::Digest, ripemd160::Ripemd160};

use ed25519_dalek::Signer;
use rand::rngs::OsRng;
use sha256::Sha256Digest;
use std::iter::repeat;

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECK_SUM_LEN: usize = 4;
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Address = String;
pub type Ripemd160Hash = [u8; 20];

#[derive(Debug)]
pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    pub fn new() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut OsRng))
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.verifying_key().to_bytes()
    }

    pub fn private_key(&self) -> PrivateKey {
        self.0.to_bytes()
    }
    pub fn address(&self) -> String {
        let hash = self.public_key().digest();
        let mut hash_160 = ripemd160_digest(hash.as_bytes()).to_vec();
        hash_160.insert(0, VERSION);
        let rt = [hash_160.clone(), checksum(hash_160.as_slice())].concat();
        base58_encode(rt.as_slice())
    }
    pub fn pub_key_hash(&self) -> Ripemd160Hash {
        let hash = self.public_key().digest();
        ripemd160_digest(hash.as_bytes()).try_into().unwrap()
    }
    pub fn prikey_hex(&self) -> String {
        hex::encode(self.private_key())
    }
    pub fn pubkey_hex(&self) -> String {
        hex::encode(self.public_key())
    }
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.0.sign(msg).into()
    }

    pub fn verify(&self, message: &[u8], signature: Signature) -> bool {
        self.0.verify(message, &signature.into()).is_ok()
    }
}

pub fn ripemd160_digest(data: &[u8]) -> Ripemd160Hash {
    let mut ripemd160 = Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);
    buf.try_into().unwrap()
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

/// 计算校验和
fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = payload.digest();
    let second_sha = first_sha.digest();
    second_sha[0..ADDRESS_CHECK_SUM_LEN].as_bytes().to_vec()
}

#[test]
fn test() {
    let key: Keypair = Keypair::new();
    let private_key = key.private_key();
    let public_key = key.public_key();
    let pub_key_hash = key.pub_key_hash();
    let address = key.address();

    let data = base58_decode(&address);
}
