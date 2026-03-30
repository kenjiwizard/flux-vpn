#!/usr/bin/env bash
# Flux VPN Build Script
# Copyright (C) 2026 Flux VPN — Created by Kenji

set -eu

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Flux VPN Build ==="
echo ""

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "Error: Rust/Cargo is required. Install from https://rustup.rs/"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Error: Node.js/npm is required."; exit 1; }

# Build daemon and CLI
echo "[1/3] Building flux-daemon and flux-cli..."
cargo build --release

# Build desktop GUI
echo "[2/3] Building desktop GUI..."
cd desktop/packages/flux-vpn
npm ci --silent
npm run build
cd "$SCRIPT_DIR"

# Package
echo "[3/3] Packaging..."
echo "Build complete! Binaries are in target/release/"
echo ""
echo "  flux-daemon  — VPN system daemon"
echo "  flux         — CLI client"
