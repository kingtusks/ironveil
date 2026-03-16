use ironveil::crypto::encode_key;
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    println!("preshared key: {}", encode_key(&key));
    println!("add this to BOTH server.toml and client.toml under [peer]");
}