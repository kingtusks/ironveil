use std::process::Command;

pub fn add_routes(server_ip: &str, gateway: &str, tun_interface: &str) -> Result<(), String> {
    run_route(&["add", server_ip, "mask", "255.255.255.255", gateway])?;
    
    run_route(&["add", "0.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
    run_route(&["add", "128.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;

    println!("routes added, all traffic going through tunnel");
    Ok(())
}

pub fn remove_routes(server_ip: &str, gateway: &str, tun_interface: &str) -> Result<(), String> {
    run_route(&["delete", server_ip, "mask", "255.255.255.255", gateway])?;
    run_route(&["delete", "0.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
    run_route(&["delete", "128.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;

    println!("routes removed");
    Ok(())
}

pub fn set_dns(tun_interface: &str, dns_server: &str) -> Result<(), String> {
    let output = Command::new("netsh")
        .args([
            "interface", "ip", "set", "dns",
            &format!("name=\"{}\"", tun_interface),
            "static", dns_server
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("set dns failed: {}", err));
    }

    println!("dns set to {} on {}", dns_server, tun_interface);
    Ok(())
}

pub fn reset_dns(tun_interface: &str) -> Result<(), String> {
    let output = Command::new("netsh")
        .args([
            "interface", "ip", "set", "dns",
            &format!("name=\"{}\"", tun_interface),
            "dhcp"
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("reset dns failed: {}", err));
    }

    println!("dns reset");
    Ok(())
}

fn run_route(args: &[&str]) -> Result<(), String> {
    let output = Command::new("route")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("route command failed: {}", err));
    }

    Ok(())
}