use tun::Configuration;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut config = Configuration::default();

    config
        .address("10.0.0.2")
        .netmask("255.255.255.0")
        .up();

    let mut dev = tun::create_as_async(&config)
        .unwrap();

    println!("tun is up");

    let mut buf = [0u8; 1504];
    loop {
        let n = dev.read(&mut buf).await.unwrap();
        
        println!("got packet: {n} bytes");
        println!("  version/ihl: {:08b}", buf[0]); 
        println!("  protocol: {}", buf[9]);           
        println!("  src: {}.{}.{}.{}", buf[12], buf[13], buf[14], buf[15]);
        println!("  dst: {}.{}.{}.{}", buf[16], buf[17], buf[18], buf[19]);
    }
}
