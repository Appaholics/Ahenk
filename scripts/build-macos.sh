#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for macOS..."

# Build for both Apple Silicon and Intel
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create output directory
mkdir -p target/macos

# Create universal binary
lipo -create \
    target/aarch64-apple-darwin/release/libnexus_core.a \
    target/x86_64-apple-darwin/release/libnexus_core.a \
    -output target/macos/libnexus_core.a

echo "macOS universal library created: target/macos/libnexus_core.a"
