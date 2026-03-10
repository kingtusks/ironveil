use boringtun::x25519::{StaticSecret, PublicKey};
use rand::rngs::OsRng;
use base64::{Engine, engine::general_purpose::STANDARD};

pub fn generate_keypair() -> (StaticSecret, PublicKey) {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    (secret, public)
}

pub fn encode_key(key: &[u8; 32]) -> String {
    STANDARD.encode(key)
}

pub fn decode_key(s: &str) -> Result<[u8; 32], String> {
    let bytes = STANDARD.decode(s).map_err(|e| e.to_string())?;
    <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| "invalid key length".to_string())
}

pub fn public_key_from_base64(s: &str) -> Result<PublicKey, String> {
    let bytes = decode_key(s)?;
    Ok(PublicKey::from(bytes))
}

pub fn secret_key_from_base64(s: &str) -> Result<StaticSecret, String> {
    let bytes = decode_key(s)?;
    Ok(StaticSecret::from(bytes))
}