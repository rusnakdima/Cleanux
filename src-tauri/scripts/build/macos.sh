#!/bin/bash
set -e

echo "Building Cleanux for macOS..."

cd "$(dirname "$0")/../.."

cargo tauri build --target aarch64-apple-darwin
cargo tauri build --target x86_64-apple-darwin

echo "macOS build complete. DMG available in:"
echo "  src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/"
echo "  src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/"
