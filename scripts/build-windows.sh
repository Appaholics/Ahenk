#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for Windows..."

# Build for x64 and ARM64
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc

# Create output directory
mkdir -p target/windows

# Copy libraries
cp target/x86_64-pc-windows-msvc/release/nexus_core.dll \
    target/windows/nexus_core_x64.dll 2>/dev/null || \
cp target/x86_64-pc-windows-msvc/release/nexus_core.lib \
    target/windows/nexus_core_x64.lib

cp target/aarch64-pc-windows-msvc/release/nexus_core.dll \
    target/windows/nexus_core_arm64.dll 2>/dev/null || \
cp target/aarch64-pc-windows-msvc/release/nexus_core.lib \
    target/windows/nexus_core_arm64.lib

echo "Windows libraries created in target/windows/"
