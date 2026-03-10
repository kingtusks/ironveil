use boringtun::noise::Tunn;
use boringtun::x25519::{StaticSecret, PublicKey};

pub fn create_tunnel(
    secret: StaticSecret,
    peer_public: PublicKey,
) -> Result<Tunn, String> {
    Tunn::new(
        secret,
        peer_public,
        None, //no preshared key
        Some(25), //keep alive every 25 seconds
        0, //tunnel index
        None, //no rate limiter
    ).map_err(|e| e.to_string())
}