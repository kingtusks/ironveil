use tokio::net::UdpSocket;
use rand::rngs::OsRng;
use tun::Configuration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use x25519_dalek::{PublicKey, StaticSecret};
use base64::{Engine, engine::general_purpose::STANDARD};
use boringtun::noise::{Tunn, TunnResult};

#[tokio::main]
async fn main() {
    let client_secret = StaticSecret::random_from_rng(OsRng);
    
    //let client_public = PublicKey::from(&client_secret);

    //let encoded = STANDARD.encode(client_public.as_bytes());
    //println!("{}", encoded);
    //client pubkey: 7Mipe/LgN56gekMQlz6bJ3EQVQyoLnqd2Nal1UWNlg8=

    let bytes = STANDARD.decode("YwNKYV92vswoJgf2Y3o84EMgkXQ8NQsDD859wqRSKns=").unwrap();
    let server_public = PublicKey::from(<[u8; 32]>::try_from(bytes.as_slice()).unwrap());

    let mut tunnel = Tunn::new(
        client_secret,
        server_public,
        None,
        Some(25),
        0,
        None
    ).unwrap();

    let mut config = Configuration::default();
    config
        .address("10.0.0.2")
        .netmask("255.255.255.0")
        .name("ironveil_client")
        .up();

    let mut dev = tun::create_as_async(&config).unwrap();
    let socket = UdpSocket::bind("0.0.0.0:51821").await.unwrap();
    let server_addr = "127.0.0.1:51820";

    let mut out_buf: [u8; 1504] = [0; 1504];
    match tunnel.encapsulate(&[], &mut out_buf) {
        TunnResult::WriteToNetwork(data) => {
            socket.send_to(data, server_addr).await.unwrap();
            println!("handshake shoke");
        }
        _ => {}
    }

    let mut tun_buf: [u8; 1504] = [0; 1504];
    let mut udp_buf: [u8; 1504] = [0; 1504];

    loop {
        tokio::select! {
            Ok(n) = dev.read(&mut tun_buf) => {
                match tunnel.encapsulate(&tun_buf[..n], &mut out_buf) {
                    TunnResult::WriteToNetwork(data) => {
                        socket.send_to(data, server_addr).await.unwrap();
                        println!("tun intercepted, encrypted and sent to serv via udp");
                    }
                    TunnResult::Err(e) => eprintln!("encapsulate err: {:?}", e),
                    _ => {}
                }
            }
            Ok((n, _)) = socket.recv_from(&mut udp_buf) => {
                match tunnel.decapsulate(None, &udp_buf[..n], &mut out_buf) {
                    TunnResult::WriteToTunnelV4(data, _) => {
                        println!("wrote decrypted ip packet into tun");
                        dev.write_all(data).await.unwrap();
                    }
                    TunnResult::WriteToNetwork(data) => {
                        println!("handshake reply sent");
                        socket.send_to(data, server_addr).await.unwrap();
                    }
                    TunnResult::Err(e) => eprintln!("decapsulate error: {:?}", e),
                    _ => {}
                }
            }
        }
    }

}