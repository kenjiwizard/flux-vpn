#!/usr/bin/env bash
# Run all CI checks locally
set -eu

echo "=== Flux VPN CI Check ==="
echo ""
echo "[1/4] Formatting..."
cargo fmt --all -- --check

echo "[2/4] Clippy..."
cargo clippy --all-targets -- -D warnings

echo "[3/4] Tests..."
cargo test --all

echo "[4/4] Desktop lint..."
cd desktop/packages/flux-vpn && npm run lint

echo ""
echo "All checks passed!"
