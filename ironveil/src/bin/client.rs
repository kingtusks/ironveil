use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use base64::{Engine, engine::general_purpose::STANDARD};

#[tokio::main]
async fn main() {
    let client_secret = StaticSecret::random_from_rng(OsRng);
    let client_public = PublicKey::from(&client_secret);

    //let encoded = STANDARD.encode(client_public.as_bytes());
    //println!("{}", encoded);
    //client pubkey: 7Mipe/LgN56gekMQlz6bJ3EQVQyoLnqd2Nal1UWNlg8=
    
    let bytes = STANDARD.decode("YwNKYV92vswoJgf2Y3o84EMgkXQ8NQsDD859wqRSKns=").unwrap();
    let server_public = PublicKey::from(<[u8; 32]>::try_from(bytes.as_slice()).unwrap());

}