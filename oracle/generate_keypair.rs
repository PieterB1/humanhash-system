use rust_secp256k1::{KeyPair, Secp256k1}; use rand::rngs::OsRng;
fn main() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let keypair = KeyPair::new(&secp, &mut rng);
    let x_only_pubkey = keypair.x_only_public_key().0;
    println!("Public key: {}", hex::encode(x_only_pubkey.serialize()));
    println!("Secret key (store securely): {}", hex::encode(keypair.secret_key().secret_bytes()));
}
