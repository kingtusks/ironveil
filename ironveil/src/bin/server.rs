use tokio::net::UdpSocket;
use rand::rngs::OsRng;
use tun::Configuration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use x25519_dalek::{PublicKey, StaticSecret};
use base64::{Engine, engine::general_purpose::STANDARD};
use boringtun::noise::{Tunn, TunnResult};

#[tokio::main]
async fn main() {
    let server_secret = StaticSecret::random_from_rng(OsRng);
    
    //let server_public = PublicKey::from(&server_secret);

    //let encoded = STANDARD.encode(server_public.as_bytes());
    //println!("{}", encoded);
    //server pubkey: YwNKYV92vswoJgf2Y3o84EMgkXQ8NQsDD859wqRSKns=

    let bytes = STANDARD.decode("7Mipe/LgN56gekMQlz6bJ3EQVQyoLnqd2Nal1UWNlg8=").unwrap();
    let client_public = PublicKey::from(<[u8; 32]>::try_from(bytes.as_slice()).unwrap());

    let mut tunnel = Tunn::new(
        server_secret,
        client_public,
        None,
        Some(25),
        0,
        None
    ).unwrap();

    let mut config = Configuration::default();
    config
        .address("10.0.0.1")
        .netmask("255.255.255.0")
        .name("ironveil_server")
        .up();
    let mut dev = tun::create_as_async(&config).unwrap();
    let socket = UdpSocket::bind("0.0.0.0:51820").await.unwrap();
    println!("server listening on port 51820");
    
    let mut tun_buf: [u8; 1504] = [0; 1504];
    let mut udp_buf: [u8; 1504] = [0; 1504];
    let mut out_buf: [u8; 1504] = [0; 1504];
    let mut peer_addr = None;

    loop {
        tokio::select! {
            Ok(n) = dev.read(&mut tun_buf) => {
                if let Some(addr) = peer_addr {
                    match tunnel.encapsulate(&tun_buf[..n], &mut out_buf) {
                        TunnResult::WriteToNetwork(data) => {
                            socket.send_to(data, addr).await.unwrap();
                            println!("tun > encrypted > udp");
                        }
                        TunnResult::Err(e) => eprintln!("encapsulate error: {:?}", e),
                        _ => {}
                    }
                }
            }
            Ok((n, addr)) = socket.recv_from(&mut udp_buf) => {
                peer_addr = Some(addr);
                match tunnel.decapsulate(None, &udp_buf[..n], &mut out_buf) {
                    TunnResult::WriteToTunnelV4(data, _) => {
                        println!("udp > decrypted > tun");
                        dev.write_all(data).await.unwrap();
                    }
                    TunnResult::WriteToNetwork(data) => {
                        println!("handshake response sent");
                        socket.send_to(data, addr).await.unwrap();
                    }
                    TunnResult::Err(e) => eprintln!("decapsulate error: {:?}", e),
                    _ => {}
                }
            }
        }
    }
}