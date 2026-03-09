use x25519_dalek::{PublicKey, StaticSecret};
use rand::rngs::OsRng;

fn main() {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    
    println!("Private: {:02x?}", secret.to_bytes());
    println!("Public: {:02x?}", public.to_bytes());
}