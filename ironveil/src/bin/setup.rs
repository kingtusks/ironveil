use std::fs;
use ironveil::keygen;

struct OutboundData {
    client_public_key: String,
}

struct InboundData {
    preshared_key: String,
    server_public_key: String,
    endpoint: String, //with port like XX.XXX.XX.XXX:XXXXX
}


fn main() {
    let keys = keygen::make_keys();
    
    /*
    (PLACEHOLDER)
    - inbound and outbound are just stubs
    */
    let inbound: InboundData = InboundData { 
        preshared_key: "".to_owned(), 
        server_public_key: "".to_owned(), 
        endpoint: "".to_owned(), 
    };

    let _outbound: OutboundData = OutboundData { client_public_key: keys.1 };

    //format macro is fine here cus base64 doesnt use curly braces (phew)
    let toml_content = format!(r#"
    [interface]
    private_key = "{client_private_key}"
    address = "10.0.0.2"

    [peer]
    public_key = "{server_public_key}"
    preshared_key = "{preshared_key}"
    endpoint = "{endpoint}"
    allowed_ips = "0.0.0.0/0"

    [routing]
    gateway = "192.168.1.1" 
    tun_name = "(PLACEHOLDER)"
    dns_server = "1.1.1.1"
    real_interface = "(PLACEHOLDER)" 
    "#,
    client_private_key = keys.0,
    server_public_key = inbound.server_public_key,
    preshared_key = inbound.preshared_key,
    endpoint = inbound.endpoint, 
    );

    fs::write("client.toml", toml_content.trim())
        .expect("failed to create client.toml");

}