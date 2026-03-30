# Architecture

## Overview

Flux VPN is composed of two main layers:

### Talpid Layer (Generic VPN Engine)

The `talpid-*` crates form a **generic, privacy-preserving VPN client library**. They have no knowledge of Flux-specific services, APIs, or accounts. This layer handles:

- Tunnel creation and management (`talpid-tunnel`, `talpid-core`)
- Firewall rules and kill switch (`talpid-firewall`)
- DNS configuration (`talpid-dns`)
- OS routing table management (`talpid-routing`)
- Network utilities (`talpid-net`)
- Shared types (`talpid-types`)

### Flux Layer (Application Logic)

The `flux-*` crates build on top of the Talpid engine to create the full Flux VPN product:

- Main daemon binary (`flux-daemon`)
- CLI frontend (`flux-cli`)
- API client (`flux-api`)
- Relay server selection (`flux-relay-selector`)
- Shared types (`flux-types`)
- DNS configuration (`flux-dns`)
- File paths (`flux-paths`)
- Logging (`flux-logging`)

### Frontends

- **Desktop GUI** — Electron + React application in `desktop/`
- **Android** — Kotlin + Jetpack Compose in `android/`
- **iOS** — Swift + SwiftUI in `ios/`
- **CLI** — Rust binary `flux` in `flux-cli/`

All frontends communicate with `flux-daemon` through a management interface.
