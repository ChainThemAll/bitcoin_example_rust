use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Account {}

#[test]
fn test() {
    /*     use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
    use hex::encode as hex_encode;
    use rand::rngs::OsRng;
    use sha256::{digest, Sha256};
    // 生成密钥对
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    // 从密钥对中提取公钥和私钥
    let public_key: PublicKey = keypair.public;
    let secret_key: SecretKey = keypair.secret;

    // 生成地址，这里简单地使用公钥的SHA256哈希作为地址
    let mut hasher = Sha256::new();
    hasher.update(public_key.as_bytes());
    let address = hex_encode(hasher.finalize());

    println!("Address: {}", address);

    // 签名消息
    let message: &[u8] = b"Hello, world!";
    let signature: Signature = keypair.sign(message);

    // 验证签名
    match public_key.verify(message, &signature) {
        Ok(_) => println!("Signature verified."),
        Err(e) => println!("Error verifying signature: {}", e),
    } */
}
