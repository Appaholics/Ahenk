#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for WatchOS..."

# Build for WatchOS device and simulator
cargo build --release --target aarch64-apple-watchos
cargo build --release --target aarch64-apple-watchos-sim

# Create output directory
mkdir -p target/watchos

# Copy device library
cp target/aarch64-apple-watchos/release/libnexus_core.a \
    target/watchos/libnexus_core_device.a

# Copy simulator library
cp target/aarch64-apple-watchos-sim/release/libnexus_core.a \
    target/watchos/libnexus_core_sim.a

echo "WatchOS libraries created:"
echo "  - target/watchos/libnexus_core_device.a (device)"
echo "  - target/watchos/libnexus_core_sim.a (simulator)"
