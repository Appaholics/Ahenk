#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for Linux..."

# Build for x86_64 and ARM64
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Create output directory
mkdir -p target/linux

# Copy libraries
cp target/x86_64-unknown-linux-gnu/release/libnexus_core.so \
    target/linux/libnexus_core_x64.so 2>/dev/null || \
cp target/x86_64-unknown-linux-gnu/release/libnexus_core.a \
    target/linux/libnexus_core_x64.a

cp target/aarch64-unknown-linux-gnu/release/libnexus_core.so \
    target/linux/libnexus_core_arm64.so 2>/dev/null || \
cp target/aarch64-unknown-linux-gnu/release/libnexus_core.a \
    target/linux/libnexus_core_arm64.a

echo "Linux libraries created in target/linux/"
