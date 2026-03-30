# Security in Flux VPN

Flux VPN is designed with a **security-first** philosophy. All defaults are configured to maximize privacy, and users must explicitly opt in to less restrictive settings.

## Threat Model

Flux VPN protects against:

- **Network surveillance** — all traffic is encrypted through WireGuard tunnels
- **DNS leaks** — all DNS queries are routed through the tunnel
- **IPv6 leaks** — IPv6 traffic is blocked unless properly tunneled
- **Traffic analysis** — DAITA (Defense Against AI-guided Traffic Analysis) is available
- **Tunnel drops** — the kill switch blocks all traffic if the tunnel goes down

## Kill Switch

The kill switch is **enabled by default** and cannot be accidentally disabled. When active, all network traffic is blocked unless it flows through the VPN tunnel. This prevents any data from leaking if the connection drops unexpectedly.

## DNS Leak Prevention

Flux VPN configures the system DNS to use tunnel-internal resolvers. No DNS queries leave the device outside of the encrypted tunnel.

## Split Tunneling

Split tunneling allows specific applications to bypass the VPN tunnel. This feature is **disabled by default** because it reduces the security posture. When enabled, only explicitly listed applications will bypass the tunnel.

## Quantum-Resistant Tunnels

Flux VPN supports post-quantum key exchange to protect tunnel traffic against future quantum computer attacks. This feature performs an additional key exchange using a quantum-resistant algorithm alongside the standard WireGuard handshake.

## Obfuscation

For users in restrictive network environments, Flux VPN supports tunneling WireGuard traffic over TCP, Shadowsocks, and QUIC to bypass deep packet inspection.
