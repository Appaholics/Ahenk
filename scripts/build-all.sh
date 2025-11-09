#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "Building nexus-core for all platforms..."

# Detect OS
OS="$(uname -s)"

case "$OS" in
    Darwin*)
        echo "Building on macOS - all platforms available"
        ./scripts/build-ios.sh
        ./scripts/build-macos.sh
        ./scripts/build-watchos.sh
        ./scripts/build-android.sh
        ./scripts/build-wearos.sh
        echo "Note: Skipping Windows/Linux (requires cross or native host)"
        ;;
    Linux*)
        echo "Building on Linux"
        ./scripts/build-linux.sh
        ./scripts/build-android.sh
        ./scripts/build-wearos.sh
        echo "Note: iOS/macOS/WatchOS require macOS host"
        echo "Note: Windows requires cross or Windows host"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "Building on Windows"
        ./scripts/build-windows.sh
        echo "Note: iOS/macOS/WatchOS require macOS host"
        echo "Note: Android/WearOS require NDK setup"
        echo "Note: Linux requires cross or Linux host"
        ;;
    *)
        echo "Unknown OS: $OS"
        exit 1
        ;;
esac

echo "Build complete! Check target/ directory for outputs"
