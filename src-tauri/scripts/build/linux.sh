#!/bin/bash
set -e

echo "Building Cleanux for Linux..."

cd "$(dirname "$0")/../.."

cargo tauri build --target x86_64-unknown-linux-gnu

echo "Linux build complete. Packages available in:"
echo "  src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/"
echo "  src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/appimage/"
echo "  src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/rpm/"
