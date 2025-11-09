#!/bin/bash
set -e

echo "Installing Rust targets for cross-compilation..."

# iOS
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim  
rustup target add x86_64-apple-ios

# macOS
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Linux (desktop)
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Windows
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc

# WatchOS
rustup target add aarch64-apple-watchos
rustup target add aarch64-apple-watchos-sim

echo "Installing build tools..."
cargo install cargo-ndk --force
cargo install cargo-lipo --force

echo "Setup complete!"
