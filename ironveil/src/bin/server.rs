use boringtun::x25519::{StaticSecret, PublicKey};
use rand::rngs::OsRng;
use base64::{Engine, engine::general_purpose::STANDARD};

#[tokio::main]
async fn main() {
    let server_secret = StaticSecret::random_from_rng(OsRng);
    let server_public = PublicKey::from(&server_secret);

    //let encoded = STANDARD.encode(server_public.as_bytes());
    //println!("{}", encoded);
    //server pubkey: YwNKYV92vswoJgf2Y3o84EMgkXQ8NQsDD859wqRSKns=

    let bytes = STANDARD.decode("7Mipe/LgN56gekMQlz6bJ3EQVQyoLnqd2Nal1UWNlg8=").unwrap();
    let client_public = PublicKey::from(<[u8; 32]>::try_from(bytes.as_slice()).unwrap());
}