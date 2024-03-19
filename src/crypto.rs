use crypto::{digest::Digest, ripemd160::Ripemd160};
use rand::rngs::OsRng;
use sha256::Sha256Digest;
use std::iter::repeat;

const VERSION: u8 = 0x00;

pub type Publickey = [u8; 32];
pub type Privatekey = [u8; 32];
pub type Address = String;

#[derive(Debug)]
pub struct Keypair(ed25519_dalek::SigningKey);

impl Keypair {
    pub fn new() -> Self {
        Self(ed25519_dalek::SigningKey::generate(&mut OsRng))
    }

    pub fn public_key(&self) -> Publickey {
        self.0.verifying_key().to_bytes()
    }

    pub fn private_key(&self) -> Privatekey {
        self.0.to_bytes()
    }
    pub fn address(&self) -> String {
        let hash = self.public_key().digest();
        let mut hash_160 = ripemd160_digest(hash.as_bytes());
        hash_160.insert(0, VERSION);
        let rt = [hash_160.clone(), checksum(hash_160.as_slice())].concat();
        base58_encode(rt.as_slice())
    }

    pub fn prikey_hex(&self) -> String {
        hex::encode(self.private_key())
    }
    pub fn pubkey_hex(&self) -> String {
        hex::encode(self.public_key())
    }
}

pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);
    buf
}

/// base58 编码
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// base58 解码
pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

/// 计算校验和
fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = payload.digest();
    let second_sha = first_sha.digest();
    second_sha[0..4].as_bytes().to_vec()
}

#[test]
fn test() {
    let keypair: Keypair = Keypair::new();

    let private_key = keypair.prikey_hex();

    let public_key = keypair.pubkey_hex();

    let address = keypair.address();

    println!("private_key: {:?}", private_key);
    println!("public_key: {:?}", public_key);
    println!("address: {:}", address);
}
