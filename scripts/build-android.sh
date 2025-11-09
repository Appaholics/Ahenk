#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# Check for Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set"
    echo "Set it to your Android NDK path, e.g.:"
    echo "export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653"
    exit 1
fi

echo "Building nexus-core for Android..."

# Build for all Android architectures
cargo ndk --target aarch64-linux-android --platform 21 build --release
cargo ndk --target armv7-linux-androideabi --platform 21 build --release
cargo ndk --target i686-linux-android --platform 21 build --release
cargo ndk --target x86_64-linux-android --platform 21 build --release

# Create JNI directory structure
mkdir -p target/android/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}

# Copy libraries to JNI structure
cp target/aarch64-linux-android/release/libnexus_core.so \
    target/android/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libnexus_core.so \
    target/android/jniLibs/armeabi-v7a/
cp target/i686-linux-android/release/libnexus_core.so \
    target/android/jniLibs/x86/
cp target/x86_64-linux-android/release/libnexus_core.so \
    target/android/jniLibs/x86_64/

echo "Android libraries created in target/android/jniLibs/"
