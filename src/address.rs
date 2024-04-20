#[allow(dead_code)]
fn getpubkey_from_private_key(private_key: &[u8; 32]) -> String {
    use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};
    let secret_key = SecretKey::from_slice(private_key).expect("secret key err");
    let public_key = secret_key.public_key().to_encoded_point(false);
    let x = hex::encode(public_key.x().unwrap().as_slice());
    let y = hex::encode(public_key.y().unwrap().as_slice());
    format!("0x04{}{}", x, y)
}
#[allow(dead_code)]
fn get_eth_address_from_pubkey(public_key: &[u8; 64]) -> String {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(public_key);
    let hash = hasher.finalize();
    format!("0x{}", hex::encode(hash)[24..].to_owned())
}
#[allow(dead_code)]
fn eip_55(address: &[u8; 20]) -> String {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(hex::encode(address));
    let hash = hasher.finalize();
    let hash = hex::encode(hash).to_owned();
    let address = hex::encode(address);
    let address = address
        .chars()
        .enumerate()
        .map(|(i, char)| {
            if char.is_ascii_lowercase() && hash.chars().nth(i).unwrap_or('0') >= '8' {
                char.to_ascii_uppercase()
            } else {
                char
            }
        })
        .collect::<String>();
    format!("0x{}", address)
}

#[allow(dead_code)]
fn get_eip55_eth_address_from_private_key(private_key: String) -> String {
    //get public key
    let private_key_whihout_prefix = private_key.trim_start_matches("0x");
    let public_key = getpubkey_from_private_key(
        hex::decode(private_key_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("private_key len err"),
    );

    //get eth address
    let public_key_whihout_prefix = public_key.trim_start_matches("0x04");
    let eth_address = get_eth_address_from_pubkey(
        hex::decode(public_key_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("public_key len err"),
    );

    //get eip55 eth address
    let eth_address_whihout_prefix = eth_address.trim_start_matches("0x");

    eip_55(
        hex::decode(eth_address_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("eth_address len err"),
    )
}

#[test]
fn test() {
    let address = get_eip55_eth_address_from_private_key(
        "0x3ac5dc9a32f4db6501f7fc01f61961e4c30efbf46f01ad73c09c113bb678e60b".to_owned(),
    );
    assert_eq!(address, "0x18a5113F7FBC31E73c2bB38a895FD6683803E7F8");
}
#[test]
fn test_gen_eth_address_from_private_key() {
    //get public key
    let private_key =
        "0xf8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315".to_owned();
    let private_key_whihout_prefix = private_key.trim_start_matches("0x");
    let public_key = getpubkey_from_private_key(
        hex::decode(private_key_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("private_key len err"),
    );

    assert_eq!(
        public_key,
        "0x046e145ccef1033dea239875dd00dfb4fee6e3348b84985c92f103444683bae07b83b5c38e5e2b0\
    c8529d7fa3f64d46daa1ece2d9ac14cab9477d042c84c32ccd0"
            .to_owned()
    );

    //get eth address
    let public_key_whihout_prefix = public_key.trim_start_matches("0x04");
    let eth_address = get_eth_address_from_pubkey(
        hex::decode(public_key_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("public_key len err"),
    );

    assert_eq!(
        eth_address,
        "0x001d3f1ef827552ae1114027bd3ecf1f086ba0f9".to_owned()
    );

    //get eip55 eth address
    let eth_address_whihout_prefix = eth_address.trim_start_matches("0x");
    let eip55_eth_address = eip_55(
        hex::decode(eth_address_whihout_prefix)
            .expect("decode err")
            .as_slice()
            .try_into()
            .expect("eth_address len err"),
    );
    assert_eq!(
        eip55_eth_address,
        "0x001d3F1ef827552Ae1114027BD3ECF1f086bA0F9".to_owned()
    );

    //test example
    assert_eq!(
        get_eip55_eth_address_from_private_key(
            "0x1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727".to_owned()
        ),
        "0x9858EfFD232B4033E47d90003D41EC34EcaEda94".to_owned()
    );
}
