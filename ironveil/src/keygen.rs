use crate::crypto::{encode_key, generate_keypair};

//(secret, public)
pub fn make_keys() -> (String, String) {
    let (secret, public) = generate_keypair();
    // println!("private: {:02x?}", encode_key(&secret.to_bytes()));
    // println!("public: {:02x?}", encode_key(&public.to_bytes()));
    (encode_key(&secret.to_bytes()), encode_key(&public.to_bytes()))
}
