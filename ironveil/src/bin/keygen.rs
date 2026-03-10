use ironveil::crypto::{encode_key, generate_keypair};

fn main() { 
    let (secret, public) = generate_keypair();

    println!("private: {:02x?}", encode_key(&secret.to_bytes()));
    println!("public: {:02x?}", encode_key(&public.to_bytes()));
}