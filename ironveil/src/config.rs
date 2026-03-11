use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub interface: Interface,
    pub peer: Peer,
    pub routing: Option<Routing>,
}

#[derive(Deserialize, Serialize)]
pub struct Interface {
    pub private_key: String, //encoded w base64
    pub address: String,     
    pub port: Option<u16>,  //listen port
}

#[derive(Deserialize, Serialize)]
pub struct Peer {
    pub public_key: String, //encoded w base64
    pub endpoint: Option<String>, 
    pub allowed_ips: String, 
}

#[derive(Deserialize, Serialize)]
pub struct Routing {
    pub gateway: String,
    pub tun_interface: String,
    pub dns_server: String,
    pub real_interface: String,
}

pub fn load(path: &str) -> Result<Config, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&text).map_err(|e| e.to_string())
}

//pub fn makeConfig() later for readability