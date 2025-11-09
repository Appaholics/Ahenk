#!/bin/bash
set -e

# WearOS uses the same targets as Android
cd "$(dirname "$0")/.."

# Check for Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set"
    exit 1
fi

echo "Building nexus-core for WearOS..."

# WearOS typically uses arm64-v8a and armeabi-v7a
cargo ndk --target aarch64-linux-android --platform 23 build --release
cargo ndk --target armv7-linux-androideabi --platform 23 build --release

# Create WearOS directory structure
mkdir -p target/wearos/jniLibs/{arm64-v8a,armeabi-v7a}

# Copy libraries
cp target/aarch64-linux-android/release/libnexus_core.so \
    target/wearos/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libnexus_core.so \
    target/wearos/jniLibs/armeabi-v7a/

echo "WearOS libraries created in target/wearos/jniLibs/"
