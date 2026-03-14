use std::process::Command;

pub fn add_routes(server_ip: &str, gateway: &str, tun_interface: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        println!("adding route: {} via {}", server_ip, gateway);
        run_route(&["add", server_ip, "mask", "255.255.255.255", gateway])?;
        println!("adding 0.0.0.0/1 via if {}", tun_interface);
        run_route(&["add", "0.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
        println!("adding 128.0.0.0/1 via if {}", tun_interface);
        run_route(&["add", "128.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
    }

    #[cfg(unix)]
    {
        run_cmd("ip", &["route", "replace", server_ip, "via", gateway])?;
        run_cmd("ip", &["route", "replace", "0.0.0.0/1", "dev", tun_interface])?;
        run_cmd("ip", &["route", "replace", "128.0.0.0/1", "dev", tun_interface])?;
        run_cmd("ip6tables", &["-I", "OUTPUT", "-j", "DROP"])?;
    }

    println!("routes added, all traffic going through tunnel");
    Ok(())
}

pub fn remove_routes(server_ip: &str, gateway: &str, tun_interface: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        run_route(&["delete", server_ip, "mask", "255.255.255.255", gateway])?;
        run_route(&["delete", "0.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
        run_route(&["delete", "128.0.0.0", "mask", "128.0.0.0", "0.0.0.0", "if", tun_interface])?;
    }

    #[cfg(unix)]
    {
        run_cmd("ip", &["route", "del", server_ip, "via", gateway])?;
        run_cmd("ip", &["route", "del", "0.0.0.0/1", "dev", tun_interface])?;
        run_cmd("ip", &["route", "del", "128.0.0.0/1", "dev", tun_interface])?;
        run_cmd("ip6tables", &["-D", "OUTPUT", "-j", "DROP"])?;
    }

    println!("routes removed");
    Ok(())
}

pub fn set_dns(tun_name: &str, dns_server: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        let output = Command::new("netsh")
            .args([
                "interface", "ip", "set", "dns",
                &format!("name=\"{}\"", tun_name),
                "static", dns_server,
            ])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("set dns failed: {}", err));
        }
    }

    #[cfg(unix)]
    {
        let result = run_cmd("resolvectl", &["dns", tun_name, dns_server]);
        if result.is_err() {
            std::fs::write("/etc/resolv.conf", format!("nameserver {}\n", dns_server))
                .map_err(|e| e.to_string())?;
        }
    }

    println!("dns set to {} on {}", dns_server, tun_name);
    Ok(())
}

pub fn reset_dns(tun_name: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        let output = Command::new("netsh")
            .args([
                "interface", "ip", "set", "dns",
                &format!("name=\"{}\"", tun_name),
                "dhcp",
            ])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("reset dns failed: {}", err));
        }
    }

    #[cfg(unix)]
    {
        let result = run_cmd("resolvectl", &["revert", tun_name]);
        if result.is_err() {
            std::fs::write("/etc/resolv.conf", "nameserver 8.8.8.8\n")
                .map_err(|e| e.to_string())?;
        }
    }

    println!("dns reset");
    Ok(())
}

pub fn enable_kill_switch(server_ip: &str) -> Result<(), String> {
    #[cfg(windows)]
    {
        run_netsh(&[
            "advfirewall", "firewall", "add", "rule",
            "name=IronVeilKillSwitchBlock",
            "dir=out", "action=block",
            "remoteip=any",
            "enable=yes",
        ])?;

        run_netsh(&[
            "advfirewall", "firewall", "add", "rule",
            "name=IronVeilKillSwitchAllow",
            "dir=out", "action=allow",
            &format!("remoteip={}", server_ip),
            "enable=yes",
        ])?;
    }

    #[cfg(unix)]
    {
        run_cmd("iptables", &["-I", "OUTPUT", "-j", "DROP"])?;
        run_cmd("iptables", &["-I", "OUTPUT", "-d", server_ip, "-j", "ACCEPT"])?;
        run_cmd("iptables", &["-I", "OUTPUT", "-o", "tun+", "-j", "ACCEPT"])?;
    }

    println!("kill switch enabled");
    Ok(())
}

pub fn disable_kill_switch() -> Result<(), String> {
    #[cfg(windows)]
    {
        run_netsh(&["advfirewall", "firewall", "delete", "rule", "name=IronVeilKillSwitchBlock"])?;
        run_netsh(&["advfirewall", "firewall", "delete", "rule", "name=IronVeilKillSwitchAllow"])?;
    }

    #[cfg(unix)]
    {
        run_cmd("iptables", &["-D", "OUTPUT", "-j", "DROP"])?;
        run_cmd("iptables", &["-D", "OUTPUT", "-o", "tun+", "-j", "ACCEPT"])?;
    }

    println!("kill switch disabled");
    Ok(())
}

pub fn get_tun_interface_index(name: &str) -> Result<u32, String> {
    #[cfg(unix)]
    {
        let name_cstr = std::ffi::CString::new(name).map_err(|e| e.to_string())?;
        let index = unsafe { libc::if_nametoindex(name_cstr.as_ptr()) };
        if index == 0 {
            return Err(format!("interface {} not found", name));
        }
        return Ok(index);
    }

    #[cfg(windows)]
    {
        let output = Command::new("powershell")
            .args(["-Command", &format!("(Get-NetAdapter -Name '{}').ifIndex", name)])
            .output()
            .map_err(|e| e.to_string())?;

        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u32>()
            .map_err(|e| e.to_string())
    }
}

#[allow(dead_code)]
fn run_cmd(program: &str, args: &[&str]) -> Result<(), String> { 
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{} failed: {}", program, err));
    }
    Ok(())
}

#[cfg(windows)]
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

#[cfg(windows)]
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