# Build Instructions

This document describes how to build Flux VPN from source on desktop platforms.

## Prerequisites

### All platforms
- **Rust** 1.80+ — install via [rustup](https://rustup.rs/)
- **Git** — for cloning and submodule management
- **Node.js** 20+ and **npm** — for building the desktop GUI

### Linux
```bash
sudo apt install build-essential pkg-config libdbus-1-dev libmnl-dev libnftnl-dev protobuf-compiler
```

### macOS
```bash
brew install protobuf
xcode-select --install
```

### Windows
- Visual Studio 2022 with C++ build tools
- [Protocol Buffers compiler](https://github.com/protocolbuffers/protobuf/releases)

## Building

### Clone the repository

```bash
git clone https://github.com/kenjiwizard/flux-vpn.git
cd flux-vpn
git submodule update --init
```

### Build the daemon and CLI

```bash
cargo build --release
```

The binaries will be in `target/release/`:
- `flux-daemon` — the system service
- `flux` — the CLI client

### Build the desktop GUI

```bash
cd desktop/packages/flux-vpn
npm install
npm run build
```

### Build installers

```bash
./build.sh
```

This runs sanity checks and produces platform-specific installers.

## Platform-specific notes

### Android

See [android/README.md](android/README.md) for Android build instructions.

### iOS

See [ios/README.md](ios/README.md) for iOS build instructions. Requires Xcode 15+.

## Verifying builds

All release builds are signed. Verification keys and instructions are available at [fluxvpn.org/open-source](https://fluxvpn.org/open-source).
