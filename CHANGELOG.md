# Changelog

All notable changes to Flux VPN will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- WireGuard tunnel support across all platforms
- Quantum-resistant tunnel negotiation
- DAITA (Defense Against AI-guided Traffic Analysis)
- Multihop routing
- WireGuard over TCP, Shadowsocks, and QUIC
- Split tunneling (Windows, Linux, macOS, Android)
- Custom DNS server configuration
- Content blockers for ads and trackers
- Kill switch enabled by default
- Desktop GUI (Electron + React)
- CLI client (`flux`)
- Android app (Kotlin)
- iOS app (Swift)

## [0.1.0] - 2026-03-28

### Added
- First public release
- Core VPN daemon (`flux-daemon`)
- Cross-platform WireGuard support
- Privacy-preserving firewall rules
- Desktop, Android, and iOS clients

---

[unreleased]: https://github.com/kenjiwizard/flux-vpn/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kenjiwizard/flux-vpn/releases/tag/v0.1.0
