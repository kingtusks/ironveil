use ironveil::config;
use ironveil::crypto::{public_key_from_base64, secret_key_from_base64};
use ironveil::tunnel::create_tunnel;
use boringtun::noise::TunnResult;
use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tun::Configuration;

#[tokio::main]
async fn main() {
    let cfg = config::load("config/server.toml")
        .expect("failed to load server config");
    let secret = secret_key_from_base64(&cfg.interface.private_key)
        .expect("invalid private key");
    let peer_public = public_key_from_base64(&cfg.peer.public_key)
        .expect("invalid peer public key");

    let mut tunnel = create_tunnel(
        secret,
        peer_public,
    ).expect("tunnel (server) couldnt be made");

    let mut config = Configuration::default();
    config
        .address(cfg.interface.address.as_str())
        .netmask("255.255.255.0")
        .name("ironveil_server")
        .up();

    let mut dev = tun::create_as_async(&config).expect("failed to make tun device");
    let port = cfg.interface.port.unwrap_or(51820);
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("failed to bind udp");

    println!("server listening on port {}", port);
    
    let mut tun_buf: [u8; 65535] = [0; 65535];
    let mut udp_buf: [u8; 65535] = [0; 65535];
    let mut out_buf: [u8; 65535] = [0; 65535];
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