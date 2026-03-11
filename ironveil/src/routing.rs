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

pub fn enable_kill_switch(server_ip: &str) -> Result<(), String> {
    run_netsh(&[
        "advfirewall", "firewall", "add", "rule",
        "name=IronVeilKillSwitchBlock",
        "dir=out", "action=block",
        "remoteip=any",
        "enable=yes"
    ])?;

    run_netsh(&[
        "advfirewall", "firewall", "add", "rule",
        "name=IronVeilKillSwitchAllow",
        "dir=out", "action=allow",
        &format!("remoteip={}", server_ip),
        "enable=yes"
    ])?;

    println!("kill switch enabled");
    Ok(())
}


pub fn disable_kill_switch() -> Result<(), String> {
    run_netsh(&["advfirewall", "firewall", "delete", "rule", "name=IronVeilKillSwitchBlock"])?;
    run_netsh(&["advfirewall", "firewall", "delete", "rule", "name=IronVeilKillSwitchAllow"])?;
    println!("kill switch disabled");
    Ok(())
}

pub fn get_tun_interface_index(name: &str) -> Result<u32, String> {
    #[cfg(unix)]
    {
        let name_cstr = std::ffi::CString::new(name)
            .map_err(|e| e.to_string())?;

        let index = unsafe {libc::if_nametoindex(name_cstr.as_ptr())};

        if index == 0 {
            Err(format!("interface {} not found", name))
        } else {
            Ok(index)
        }
    }

    #[cfg(windows)]
    {
        let output = std::process::Command::new("powershell")
            .args(["-Command", &format!("(Get-NetAdapter -Name '{}').ifIndex", name)])
            .output()
            .map_err(|e| e.to_string())?;

        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u32>()
            .map_err(|e| e.to_string())
    }    
}

fn run_netsh(args: &[&str]) -> Result<(), String> {
    let output = Command::new("netsh")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        let out = String::from_utf8_lossy(&output.stdout);
        return Err(format!("{} {}", err, out));
    }
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