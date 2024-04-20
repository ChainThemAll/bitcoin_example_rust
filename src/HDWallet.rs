use bip39::{Language, Mnemonic};

use core::hash;
use std::borrow::Cow;
struct WalletDemo {
    user_name: String,
    pwd: String,
    mnemonic: Mnemonic,
}

fn new_wallet() {}

#[test]
pub fn mnemonic_to_seed() {
    use bip39::rand_core;
    // gen new mnemonic

    //Define mnemonic number, language and password
    let word_count = 8 * 3;
    let language = Language::SimplifiedChinese;
    let pwd = "pwd".to_string();

    //gen entropy
    let mut entropy = [0u8; 32];
    let mut rng = rand::thread_rng();
    rand_core::RngCore::fill_bytes(&mut rng, &mut entropy[0..(word_count / 3) * 4]);

    //get mnemonic
    let mnemonic =
        Mnemonic::from_entropy_in(language, &entropy[0..(word_count / 3) * 4]).expect("mnemon err");

    let v = mnemonic.word_iter().collect::<Vec<_>>();
    println!(" mnemonic is: {:?}", v);

    //utf8_pwd_check
    let normalized_passphrase = {
        let mut cow: Cow<'_, str> = pwd.into();
        Mnemonic::normalize_utf8_cow(&mut cow);
        cow
    };
    //gen seed use PBKDF2-HMAC-SHA512
    let seed = mnemonic.to_seed_normalized(&normalized_passphrase);
    println!("seed is: {:?}", seed)
}
