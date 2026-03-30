#!/usr/bin/env bash
# Create a new Flux VPN release
set -eu

VERSION="${1:?Usage: ./scripts/release.sh <version>}"

echo "Preparing release v${VERSION}..."
git tag -s "v${VERSION}" -m "Release v${VERSION}"
echo "Tagged v${VERSION}. Push with: git push origin v${VERSION}"
