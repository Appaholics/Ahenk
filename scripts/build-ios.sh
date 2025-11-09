#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for iOS..."

# Build for all iOS architectures
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios

# Create output directory
mkdir -p target/ios

# Create universal library for simulator
lipo -create \
    target/aarch64-apple-ios-sim/release/libnexus_core.a \
    target/x86_64-apple-ios/release/libnexus_core.a \
    -output target/ios/libnexus_core_sim.a

# Copy device library
cp target/aarch64-apple-ios/release/libnexus_core.a \
    target/ios/libnexus_core_device.a

echo "iOS libraries created:"
echo "  - target/ios/libnexus_core_device.a (device)"
echo "  - target/ios/libnexus_core_sim.a (simulator)"
