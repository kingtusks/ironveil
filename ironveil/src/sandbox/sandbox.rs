use boringtun::x25519::{StaticSecret, PublicKey};
use rand::rngs::OsRng;
use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt};
use tun::Configuration;

#[tokio::main]
async fn main() {
    let secret = StaticSecret::random_from_rng(OsRng);
    let public = PublicKey::from(&secret);
    println!("public key: {:?}", public);

    let mut config = Configuration::default();

    config
        .address("10.0.0.2")
        .netmask("255.255.255.0")
        .up();

    let mut dev = tun::create_as_async(&config)
        .unwrap();
    println!("tun is up");
    
    let socket = UdpSocket::bind("0.0.0.0:51820")
        .await
        .unwrap();
    println!("udp is up");

    let mut tun_buf = [0u8; 1504];
    let mut udp_buf = [0u8; 1504];

    loop {
        tokio::select! {
            Ok(n) = dev.read(&mut tun_buf) => {
                println!("tun packet: {} bytes", n);
                println!(" protocol: {}", tun_buf[9]);
            }
            Ok((n, addr)) = socket.recv_from(&mut udp_buf) => {
                println!("udp packet: {} bytes", n);
                println!(" from: {}", addr);
            }
        }

    }
}
