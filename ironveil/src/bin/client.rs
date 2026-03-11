use ironveil::config;
use ironveil::crypto::{public_key_from_base64, secret_key_from_base64};
use ironveil::tunnel::create_tunnel;
use ironveil::routing;
use boringtun::noise::TunnResult;
use tokio::signal;
use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tun::Configuration;

/*reminder for me to work on DNS leak prevention thanks*/

#[tokio::main]
async fn main() {
    let cfg = config::load("config/client.toml")
        .expect("failed to load client config");
    let secret = secret_key_from_base64(&cfg.interface.private_key)
        .expect("invalid private key");
    let peer_public = public_key_from_base64(&cfg.peer.public_key)
        .expect("invalid peer public key");

    let mut tunnel = create_tunnel(secret, peer_public)
        .expect("tunnel (client) couldn't be made");

    let mut tun_config = Configuration::default();
    tun_config
        .address(cfg.interface.address.as_str())
        .netmask("255.255.255.0")
        .name("ironveil_client")
        .up();

    let mut dev = tun::create_as_async(&tun_config)
        .expect("failed to make tun device");
    let socket = UdpSocket::bind("0.0.0.0:51821")
        .await
        .expect("failed to bind udp");
    let server_addr = cfg.peer.endpoint
        .expect("missing endpoint in client config");

    println!("client up: shaking hands with: {}", server_addr);

    let mut out_buf: [u8; 1504] = [0; 1504];
    match tunnel.encapsulate(&[], &mut out_buf) {
        TunnResult::WriteToNetwork(data) => {
            socket.send_to(data, &server_addr).await.unwrap();
            println!("handshake shoke");
        }
        _ => {}
    }

    let routing = cfg.routing.expect("missing routing in client.toml");

    routing::add_routes(
        &server_addr,
        &routing.gateway,
        &routing.tun_interface,
    ).expect("failed to add routes");

    routing::set_dns(
        &routing.tun_interface,
        &routing.dns_server
    ).expect("failed to set dns");

    let tun_iface = routing.tun_interface.clone();
    let gateway = routing.gateway.clone();
    let server = server_addr.clone();

    tokio::spawn(async move { //dis is for cleanups when down
        signal::ctrl_c().await.expect("failed to listen for ctrl+c");
        println!("shutting down and cleaning up routes");
        routing::remove_routes(&server, &gateway, &tun_iface).ok();
        routing::reset_dns(&tun_iface).ok();
        println!("done cya");
        std::process::exit(0); 
    });

    let mut tun_buf: [u8; 1504] = [0; 1504];
    let mut udp_buf: [u8; 1504] = [0; 1504];

    loop {
        tokio::select! {
            Ok(n) = dev.read(&mut tun_buf) => {
                match tunnel.encapsulate(&tun_buf[..n], &mut out_buf) {
                    TunnResult::WriteToNetwork(data) => {
                        socket.send_to(data, &server_addr).await.unwrap();
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
                        socket.send_to(data, &server_addr).await.unwrap();
                    }

                    TunnResult::Err(e) => eprintln!("decapsulate error: {:?}", e),
                    _ => {}
                }
            }
        }
    }
}