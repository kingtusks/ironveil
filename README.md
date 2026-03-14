# IronVeil

A WireGuard-based VPN built from scratch in Rust using [boringtun](https://github.com/cloudflare/boringtun) (Cloudflare's userspace WireGuard implementation).

## Features

- WireGuard protocol via boringtun (ChaCha20-Poly1305 encryption, Curve25519 key exchange)
- Cross-platform: Windows and Linux
- Full traffic routing through tunnel (split into 0.0.0.0/1 + 128.0.0.0/1)
- DNS leak prevention
- IPv6 leak prevention
- Kill switch (blocks all traffic if tunnel drops)
- TOML-based config files
- Graceful shutdown with route/DNS cleanup

## Project Structure

```
ironveil/
├── Cargo.toml
├── quickstart.ps1          # Windows launcher (auto-elevates to admin)
├── quickstart.sh           # Linux launcher (auto-elevates to root)
├── config/
│   ├── server.example.toml
│   └── client.example.toml
├── sandbox/
│   └── test_tun.rs         # TUN/UDP validation scratch file
└── src/
    ├── lib.rs
    ├── crypto.rs            # Key generation and base64 encoding/decoding
    ├── config.rs            # TOML config loading
    ├── tunnel.rs            # boringtun tunnel setup
    ├── routing.rs           # Route management, DNS, kill switch
    └── bin/
        ├── keygen.rs        # Generate a keypair
        ├── server.rs        # VPN server (run on VPS)
        └── client.rs        # VPN client (run on your machine)
```

## Dependencies

- [boringtun](https://github.com/cloudflare/boringtun) — Cloudflare's userspace WireGuard
- [tun](https://crates.io/crates/tun) — TUN device management
- [tokio](https://tokio.rs) — Async runtime
- [x25519-dalek](https://crates.io/crates/x25519-dalek) — Curve25519 key exchange
- [serde](https://serde.rs) + [toml](https://crates.io/crates/toml) — Config parsing
- [zeroize](https://crates.io/crates/zeroize) — Secure key memory wiping

## Setup

### 1. Generate keypairs

Run keygen once on each machine (server and client):

```bash
cargo run --bin keygen
```

### 2. Configure

Copy the example configs and fill in your keys:

```bash
cp config/server.example.toml config/server.toml
cp config/client.example.toml config/client.toml
```

**`config/server.toml`** (on your VPS):
```toml
[interface]
private_key = "your_server_private_key"
address = "10.0.0.1"
port = 51820

[peer]
public_key = "your_client_public_key"
allowed_ips = "10.0.0.2/32"
```

**`config/client.toml`** (on your machine):
```toml
[interface]
private_key = "your_client_private_key"
address = "10.0.0.2"

[peer]
public_key = "your_server_public_key"
endpoint = "your.vps.ip:51820"
allowed_ips = "0.0.0.0/0"

[routing]
gateway = "192.168.1.1"       # your local gateway (ip route show default)
tun_name = "ironveil_client"
dns_server = "1.1.1.1"
real_interface = "eno1"       # your real network interface
```

### 3. VPS setup

On your VPS, enable IP forwarding and NAT:

```bash
echo "net.ipv4.ip_forward=1" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo iptables -A FORWARD -i ironveil_server -o eth0 -j ACCEPT
sudo iptables -A FORWARD -i eth0 -o ironveil_server -j ACCEPT
sudo ufw allow 51820/udp
```

### 4. Run

**Server (on VPS):**
```bash
sudo ~/.cargo/bin/cargo run --bin server
```

**Client (on your machine):**

Windows:
```powershell
.\quickstart.ps1 client
```

Linux:
```bash
sudo -E env PATH=$PATH cargo run --bin client
```

Or use quickstart:
```bash
./quickstart.sh client
```

## Quickstart

```bash
# run server only
./quickstart.sh server

# run client only
./quickstart.sh client

# generate a keypair
./quickstart.sh keygen

# manually clean up routes if something goes wrong
./quickstart.sh cleanup
```

## Security Notes

- Private keys are stored in config files — add `config/server.toml` and `config/client.toml` to `.gitignore`
- On Linux, set config file permissions: `chmod 600 config/*.toml`
- Requires root/admin to create TUN devices and modify routing tables
- Uses Wintun driver on Windows (included via the `tun` crate)

## How It Works

```
your app → OS → TUN device → boringtun (encrypt) → UDP → VPS → internet
                           ← boringtun (decrypt) ←
```

1. A TUN virtual network interface intercepts your traffic
2. boringtun encrypts packets using the WireGuard protocol
3. Encrypted packets are sent over UDP to the VPS
4. The VPS decrypts and forwards to the internet
5. Replies come back the same way in reverse

Routes are split into two /1 blocks (`0.0.0.0/1` and `128.0.0.0/1`) which together cover all IPs but are more specific than the default route, so they take priority without breaking the route to the VPS itself.