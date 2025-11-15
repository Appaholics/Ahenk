# Cross-Compilation Quick Reference

## Quick Start

```bash
cd ahenk

# First time setup
make setup-targets

# Build for specific platform
make build-ios
make build-android
make build-macos
make build-windows
make build-watchos
make build-wearos

# Build all (host-dependent)
make build-all
```

## Platform Matrix

| Platform | Host Required | Architectures | Output |
|----------|--------------|---------------|--------|
| iOS | macOS | arm64, arm64-sim, x86_64-sim | `.a` static libs |
| Android | Any (with NDK) | arm64-v8a, armeabi-v7a, x86, x86_64 | `.so` shared libs |
| macOS | macOS | arm64, x86_64 | `.a` universal lib |
| Linux | Linux/Cross | x86_64, arm64 | `.so` / `.a` libs |
| Windows | Windows/Cross | x64, arm64 | `.dll` / `.lib` |
| WatchOS | macOS | arm64, arm64-sim | `.a` static libs |
| WearOS | Any (with NDK) | arm64-v8a, armeabi-v7a | `.so` shared libs |

## Output Directories

```
ahenk/target/
├── ios/
│   ├── libnexus_core_device.a
│   └── libnexus_core_sim.a
├── android/jniLibs/
│   ├── arm64-v8a/libnexus_core.so
│   ├── armeabi-v7a/libnexus_core.so
│   ├── x86/libnexus_core.so
│   └── x86_64/libnexus_core.so
├── macos/
│   └── libnexus_core.a
├── linux/
│   ├── libnexus_core_x64.so
│   └── libnexus_core_arm64.so
├── windows/
│   ├── nexus_core_x64.dll
│   └── nexus_core_arm64.dll
├── watchos/
│   ├── libnexus_core_device.a
│   └── libnexus_core_sim.a
└── wearos/jniLibs/
    ├── arm64-v8a/libnexus_core.so
    └── armeabi-v7a/libnexus_core.so
```

## Environment Variables

### Android/WearOS
```bash
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
```

### iOS/macOS/WatchOS
No environment variables needed (uses Xcode toolchain).

### Linux Cross-Compilation
```bash
# For ARM64 cross-compilation on x86_64
sudo apt-get install gcc-aarch64-linux-gnu
# or use cross tool
cargo install cross
cross build --target aarch64-unknown-linux-gnu
```

### Windows Cross-Compilation
```bash
# Using cross tool
cargo install cross
cross build --target x86_64-pc-windows-msvc
```

## Supported Targets

### iOS
- `aarch64-apple-ios` - Device (iPhone, iPad)
- `aarch64-apple-ios-sim` - Simulator (Apple Silicon Mac)
- `x86_64-apple-ios` - Simulator (Intel Mac)

### Android
- `aarch64-linux-android` - 64-bit ARM (most modern devices)
- `armv7-linux-androideabi` - 32-bit ARM (older devices)
- `i686-linux-android` - 32-bit x86 (emulators)
- `x86_64-linux-android` - 64-bit x86 (emulators)

### macOS
- `aarch64-apple-darwin` - Apple Silicon (M1/M2/M3)
- `x86_64-apple-darwin` - Intel Macs

### Windows
- `x86_64-pc-windows-msvc` - 64-bit Windows (Intel/AMD)
- `aarch64-pc-windows-msvc` - ARM64 Windows (Surface Pro X, etc.)

### WatchOS
- `aarch64-apple-watchos` - Device (Apple Watch)
- `aarch64-apple-watchos-sim` - Simulator

### WearOS
Uses same Android targets (API 23+):
- `aarch64-linux-android` - 64-bit ARM
- `armv7-linux-androideabi` - 32-bit ARM

## CI/CD

GitHub Actions workflow automatically builds for all platforms:
- `.github/workflows/build-native.yml`

Artifacts are uploaded and can be downloaded after successful builds.

## Documentation

- Full guide: [docs/cross-compilation.md](../docs/cross-compilation.md)
- Scripts README: [scripts/README.md](scripts/README.md)

## Common Issues

### "linker not found"
Install target toolchain: `rustup target add <target>`

### Android NDK issues
```bash
# Set correct NDK path
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
# Verify
ls $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/*/bin/aarch64-linux-android*-clang
```

### macOS "cannot find lipo"
Install Xcode Command Line Tools: `xcode-select --install`

### SQLite linking errors
Add to dependencies in Cargo.toml:
```toml
rusqlite = { version = "0.37.0", features = ["bundled"] }
```

## Size Optimization

The `[profile.release]` in Cargo.toml is already optimized:
- `opt-level = "z"` - Minimum binary size
- `lto = true` - Link-time optimization
- `strip = true` - Remove debug symbols
- `panic = "abort"` - Reduce panic handling overhead

For even smaller sizes, use UPX:
```bash
upx --best --lzma target/release/libnexus_core.so
```
