# Build Scripts

Cross-compilation scripts for building nexus-core on multiple platforms.

## Quick Start

```bash
# First-time setup - install all required Rust targets
./setup-targets.sh

# Build for specific platform
./build-ios.sh
./build-android.sh
./build-macos.sh
./build-windows.sh
./build-watchos.sh
./build-wearos.sh

# Build for all supported platforms on current host
./build-all.sh
```

## Requirements

See [docs/cross-compilation.md](../../docs/cross-compilation.md) for detailed prerequisites.

### Quick Setup

**iOS/macOS/WatchOS:**
- Requires macOS host
- Xcode 14+ with Command Line Tools

**Android/WearOS:**
```bash
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
```

**Windows:**
- Install MSVC toolchain or use `cross` tool

## Output Locations

- iOS: `target/ios/`
- Android: `target/android/jniLibs/`
- macOS: `target/macos/`
- Linux: `target/linux/`
- Windows: `target/windows/`
- WatchOS: `target/watchos/`
- WearOS: `target/wearos/jniLibs/`

## Troubleshooting

Run individual builds with more verbose output:
```bash
RUST_LOG=debug ./build-ios.sh
```

Check installed targets:
```bash
rustup target list --installed
```
