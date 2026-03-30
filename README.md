<p align="center">
  <img src="graphics/flux-banner.svg" width="600" alt="Flux VPN" />
</p>

<p align="center">
  <b>Free, open-source, privacy-first VPN client for desktop & mobile.</b><br/>
  <sub>Built by <a href="https://x.com/wizard_0x">Kenji</a> · Powered by WireGuard®</sub>
</p>

<p align="center">
  <a href="https://fluxvpn.org">🌐 Website</a>&nbsp;&nbsp;·&nbsp;&nbsp;
  <a href="https://x.com/flux_vpn">𝕏 Flux VPN</a>&nbsp;&nbsp;·&nbsp;&nbsp;
  <a href="https://x.com/wizard_0x">𝕏 Kenji</a>&nbsp;&nbsp;·&nbsp;&nbsp;
  <a href="https://github.com/kenjiwizard/flux-vpn/releases">📦 Downloads</a>&nbsp;&nbsp;·&nbsp;&nbsp;
  <a href="docs/">📖 Docs</a>
</p>

<p align="center">
  <a href="https://github.com/kenjiwizard/flux-vpn/releases/latest">
    <img src="https://img.shields.io/github/v/release/kenjiwizard/flux-vpn?style=flat-square&color=6d5acd&label=release" alt="Release" />
  </a>&nbsp;
  <a href="LICENSE.md">
    <img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square" alt="License" />
  </a>&nbsp;
  <a href="https://github.com/kenjiwizard/flux-vpn/stargazers">
    <img src="https://img.shields.io/github/stars/kenjiwizard/flux-vpn?style=flat-square&color=f5c542" alt="Stars" />
  </a>
</p>

---

## What is Flux VPN?

Flux VPN is a **completely free**, open-source VPN client that prioritizes your privacy, security, and anonymity. No accounts, no logs, no ads — just a clean, fast, and secure tunnel between you and the internet.

The app ships with a system daemon (`flux-daemon`), a desktop GUI built on Electron + React, a powerful CLI (`flux-cli`), and native mobile apps for Android and iOS — all from a single codebase.

> Flux VPN is and always will be **free to use**. If you find it valuable, please consider [donating](#-donate) to help keep the project alive.

---

## ✨ Features

| Feature | Windows | Linux | macOS | Android | iOS |
|:--------|:-------:|:-----:|:-----:|:-------:|:---:|
| **WireGuard** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Quantum-resistant tunnels** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **DAITA** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Multihop routing** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WireGuard over TCP** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WireGuard over Shadowsocks** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WireGuard over QUIC** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Split tunneling** | ✅ | ✅ | ✅ | ✅ | — |
| **Custom DNS** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Content blockers** (ads, trackers) | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Kill switch** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Local network access** | ✅ | ✅ | ✅ | ✅ | ✅* |

<sub>*Local network is always accessible on iOS with the current implementation.</sub>

---

## 🖥️ Supported Platforms

| OS / Platform | Supported Versions |
|:---|:---|
| **Windows** | 10, 11 |
| **macOS** | Three latest major releases |
| **Linux (Ubuntu)** | Two latest LTS + latest non-LTS |
| **Linux (Fedora)** | All non-EOL versions |
| **Linux (Debian)** | 11+ |
| **Android** | 7.0+ |
| **iOS** | 15.0+ |

---

## 🚀 Quick Start

### Download a release

Grab the latest build for your platform from the [Releases](https://github.com/kenjiwizard/flux-vpn/releases) page or from [fluxvpn.org](https://fluxvpn.org).

### Build from source

```bash
# Clone the repository
git clone https://github.com/kenjiwizard/flux-vpn.git
cd flux-vpn
git submodule update --init

# Build the daemon + CLI (requires Rust 1.80+)
cargo build --release

# Build the desktop GUI
cd desktop/packages/flux-vpn
npm install
npm run build
```

For detailed build instructions, see [BuildInstructions.md](BuildInstructions.md).

For Android, see the [Android build guide](android/README.md).
For iOS, see the [iOS build guide](ios/README.md).

---

## 🏗️ Architecture

Flux VPN is split into two layers:

```
┌──────────────────────────────────────────────┐
│              FLUX LAYER                      │
│  flux-daemon · flux-cli · flux-api           │
│  flux-types · flux-relay-selector            │
│  Flux-specific logic, API, account mgmt      │
├──────────────────────────────────────────────┤
│              TALPID LAYER                    │
│  talpid-core · talpid-tunnel · talpid-dns    │
│  talpid-firewall · talpid-routing            │
│  Generic, privacy-preserving VPN engine       │
└──────────────────────────────────────────────┘
```

The **talpid** crates form a generic, privacy-preserving VPN client library — completely agnostic to Flux-specific logic. The **flux** crates build on top of talpid to create the full Flux VPN experience.

---

## 📂 Repository Structure

```
flux-vpn/
├── flux-daemon/          # Main daemon binary (Rust)
├── flux-cli/             # Command-line interface (Rust)
├── flux-api/             # API client for Flux services
├── flux-types/           # Shared type definitions
├── flux-relay-selector/  # Server selection logic
├── flux-dns/             # DNS configuration
├── flux-logging/         # Logging utilities
├── flux-paths/           # Platform-specific file paths
├── flux-tunnel/          # Tunnel management
├── flux-wireguard/       # WireGuard integration
│
├── talpid-core/          # Core VPN client library
├── talpid-tunnel/        # Tunnel abstraction layer
├── talpid-firewall/      # Firewall management
├── talpid-dns/           # DNS resolver
├── talpid-routing/       # OS routing table management
├── talpid-net/           # Network utilities
├── talpid-types/         # Talpid type definitions
├── tunnel-obfuscation/   # Traffic obfuscation protocols
│
├── desktop/              # Electron + React desktop GUI
│   └── packages/flux-vpn/
│       ├── src/main/     # Electron main process
│       ├── src/renderer/ # React renderer
│       └── assets/       # Icons, styles, images
│
├── android/              # Android app (Kotlin)
├── ios/                  # iOS app (Swift)
│
├── dist-assets/          # Installer resources & bundled binaries
├── building/             # Build containers & tooling
├── ci/                   # CI/CD pipeline configs
├── docs/                 # Documentation
├── audits/               # Security audit reports
├── graphics/             # Logos, icons, brand assets
├── scripts/              # Dev & release helper scripts
└── test/                 # Integration & end-to-end tests
```

---

## 🔐 Security

Flux VPN is a privacy-preserving VPN client. Every setting defaults to the most secure option — users must explicitly opt into less restrictive rules.

**What we do:**

- **Kill switch by default** — all traffic is blocked if the tunnel drops
- **DNS leak prevention** — all DNS queries go through the tunnel
- **IPv6 leak prevention** — IPv6 is blocked unless properly tunneled
- **No logging** — we never log your traffic, connections, or metadata
- **Signed releases** — every build is cryptographically signed

For full details, see the [Security Policy](SECURITY.md) and the [security documentation](docs/security.md).

### Reporting Vulnerabilities

We welcome responsible disclosure. Please see [SECURITY.md](SECURITY.md) for how to report vulnerabilities.

---

## 🔧 Environment Variables

### Daemon

| Variable | Description |
|:---|:---|
| `TALPID_FIREWALL_DEBUG` | Enable firewall debug logging (`1` on Linux, `all`/`pass`/`drop` on macOS) |
| `TALPID_DNS_MODULE` | Force DNS config method: `static-file`, `resolvconf`, `systemd`, `network-manager` |
| `TALPID_FORCE_USERSPACE_WIREGUARD` | Force userspace WireGuard implementation |
| `TALPID_DISABLE_OFFLINE_MONITOR` | Always assume host is online |
| `FLUX_MANAGEMENT_SOCKET_GROUP` | Restrict management socket to a specific user group |

### Desktop GUI

| Variable | Description |
|:---|:---|
| `FLUX_PATH` | Override path to `flux-problem-report` tool |
| `FLUX_DISABLE_UPDATE_NOTIFICATION` | Set to `1` to suppress update notifications |

---

## 🖥️ Desktop Development

```bash
cd desktop/packages/flux-vpn

# Start development with hot-reload
npm run develop

# Lint the codebase
npm run lint

# Package for your platform
npm run pack:linux   # or pack:mac, pack:win

# Run tests
npm test
```

### Tray icon on Linux

If the tray icon doesn't appear on GNOME, install the [AppIndicator extension](https://extensions.gnome.org/extension/615/appindicator-support/). For other DEs, install `libappindicator3-1` or `libappindicator` via your package manager.

---

## 📁 File Paths

### Daemon paths

| Purpose | Linux | macOS | Windows |
|:---|:---|:---|:---|
| Settings | `/etc/flux-vpn/` | `/etc/flux-vpn/` | `%LOCALAPPDATA%\Flux VPN\` |
| Logs | `/var/log/flux-vpn/` | `/var/log/flux-vpn/` | `C:\ProgramData\Flux VPN\` |
| Cache | `/var/cache/flux-vpn/` | `/Library/Caches/flux-vpn/` | `C:\ProgramData\Flux VPN\cache` |
| RPC socket | `/var/run/flux-vpn` | `/var/run/flux-vpn` | `//./pipe/Flux VPN` |

### GUI settings

| Platform | Path |
|:---|:---|
| Linux | `$XDG_CONFIG_HOME/Flux VPN/gui_settings.json` |
| macOS | `~/Library/Application Support/Flux VPN/gui_settings.json` |
| Windows | `%LOCALAPPDATA%\Flux VPN\gui_settings.json` |

---

## 📖 Vocabulary

| Term | Meaning |
|:---|:---|
| **App** | The entire Flux VPN product — daemon, GUI, CLI, and mobile apps |
| **Daemon** | The `flux-daemon` background service that manages tunnels and firewall rules |
| **Frontend** | Any program connecting to the daemon's management interface (GUI or CLI) |
| **GUI** | The Electron + React graphical frontend |
| **CLI** | The `flux` terminal-based frontend |

---

## 💜 Donate

Flux VPN is **100% free** — no subscriptions, no accounts, no premium tiers. Development and server infrastructure are funded entirely by community donations.

If Flux VPN helps you stay private and secure, please consider contributing:

| Currency | Address |
|:---------|:--------|
| **SOL** | `9cRrxqDWSnpgo5rmA1sH5SfWSJv1rCvJ6wC73HyG3AWT` |
| **BTC** | `bc1qjg9627hjcxlp4g3jyhaztwe8wgxj6rl04x04nv` |
| **ETH** | `0xC1cB90fB531E2670078b50Dbb96E4cAD99C73707` |

Every donation — no matter the size — helps keep Flux VPN alive and free for everyone. Thank you. 🙏

---

## 🤝 Contributing

We welcome contributions from everyone! See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

Whether it's fixing a typo, improving docs, squashing bugs, or building new features — every PR matters.

---

## 📜 License

Copyright © 2026 Flux VPN — Created by [Kenji](https://x.com/wizard_0x)

This program is free software: you can redistribute it and/or modify it under the terms of the [GNU General Public License v3.0](LICENSE.md) as published by the Free Software Foundation.

---

<p align="center">
  <sub>
    <a href="https://fluxvpn.org">fluxvpn.org</a>&nbsp;&nbsp;·&nbsp;&nbsp;
    <a href="https://x.com/flux_vpn">𝕏 @flux_vpn</a>&nbsp;&nbsp;·&nbsp;&nbsp;
    <a href="https://x.com/wizard_0x">𝕏 @wizard_0x</a>&nbsp;&nbsp;·&nbsp;&nbsp;
    <a href="https://github.com/kenjiwizard/flux-vpn">GitHub</a>
  </sub>
</p>
