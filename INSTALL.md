# Installation Guide

This guide covers different ways to install and use `nexus-core` in your projects.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
  - [From crates.io](#from-cratesio)
  - [From Source](#from-source)
  - [For Development](#for-development)
- [Platform-Specific Setup](#platform-specific-setup)
- [FFI Integration](#ffi-integration)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required

- **Rust Nightly** (2024 edition features required)
  ```bash
  rustup install nightly
  rustup default nightly
  ```

- **Rust Version**: 1.80.0 or higher
  ```bash
  rustc --version
  ```

### System Dependencies

nexus-core uses bundled SQLite, so no separate SQLite installation is required. However, some platforms may need additional dependencies:

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc pkg-config openssl-devel

# Arch Linux
sudo pacman -S base-devel openssl
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ development tools
- Or install [Rust with MSVC toolchain](https://www.rust-lang.org/tools/install)

## Installation Methods

### From crates.io

Add `nexus-core` to your `Cargo.toml`:

```toml
[dependencies]
nexus-core = "0.1.0"
```

Then run:
```bash
cargo build
```

### From Source

Clone the repository and build:

```bash
git clone https://github.com/kodfikirsanat/focussuite.git
cd focussuite/nexus-core
cargo build --release
```

### For Development

For local development with the latest changes:

```toml
[dependencies]
nexus-core = { path = "../nexus-core" }
```

Or use as a git dependency:

```toml
[dependencies]
nexus-core = { git = "https://github.com/kodfikirsanat/focussuite", branch = "main" }
```

## Platform-Specific Setup

### iOS Development

Install iOS targets:
```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
```

Build for iOS:
```bash
cargo build --target aarch64-apple-ios --release
```

### Android Development

1. Install Android NDK (recommended version 25.2.9519653):
   ```bash
   # Using Android Studio SDK Manager or
   # Download from https://developer.android.com/ndk/downloads
   ```

2. Set environment variable:
   ```bash
   export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
   ```

3. Install Android targets:
   ```bash
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add i686-linux-android
   rustup target add x86_64-linux-android
   ```

4. Build for Android:
   ```bash
   cargo build --target aarch64-linux-android --release
   ```

### Cross-Compilation

For detailed cross-compilation instructions, see [CROSS_COMPILATION.md](CROSS_COMPILATION.md).

## FFI Integration

### C/C++ Projects

nexus-core provides a C-compatible FFI interface:

1. Build as a C library:
   ```bash
   cargo build --release
   ```

2. Link against the library:
   - Static: `target/release/libnexus_core.a` (Unix) or `nexus_core.lib` (Windows)
   - Dynamic: `target/release/libnexus_core.so` (Linux), `libnexus_core.dylib` (macOS), or `nexus_core.dll` (Windows)

3. Include the header (you'll need to generate bindings using cbindgen or similar tools)

### React Native / JavaScript

For React Native integration:

1. Build for your target platforms (iOS/Android)
2. Use FFI bindings (e.g., via JSI or a bridge library)
3. See FocusSuite mobile app for reference implementation

### Swift/Objective-C (iOS)

1. Build the static library:
   ```bash
   cargo build --target aarch64-apple-ios --release
   ```

2. Add the library to your Xcode project
3. Create Swift/Objective-C bindings (use cbindgen for header generation)

### Java/Kotlin (Android)

1. Build Android libraries:
   ```bash
   cargo build --target aarch64-linux-android --release
   ```

2. Copy libraries to `jniLibs` folder:
   ```
   app/src/main/jniLibs/
   ├── arm64-v8a/libnexus_core.so
   ├── armeabi-v7a/libnexus_core.so
   ├── x86/libnexus_core.so
   └── x86_64/libnexus_core.so
   ```

3. Use JNI or JNA for bindings

## Optional Features

### Tauri Integration

Enable Tauri support:

```toml
[dependencies]
nexus-core = { version = "0.1.0", features = ["tauri-api"] }
```

## Verification

After installation, verify it works:

```rust
use nexus_core::{initialize_database, register_user};

fn main() -> Result<(), nexus_core::NexusError> {
    // Initialize database
    let conn = initialize_database(":memory:")?;

    // Register a test user
    let user = register_user(
        &conn,
        "testuser".to_string(),
        "test@example.com".to_string(),
        "password123".to_string(),
    )?;

    println!("User created: {}", user.user_name);
    Ok(())
}
```

Run tests to ensure everything is working:
```bash
cargo test
```

## Troubleshooting

### "edition 2024 is unstable"

Make sure you're using Rust nightly:
```bash
rustup default nightly
cargo --version  # Should show "nightly"
```

### "linker not found" errors

Install the appropriate toolchain:
```bash
# For the specific target
rustup target add <your-target>
```

### OpenSSL errors (Linux)

Install OpenSSL development packages:
```bash
sudo apt-get install libssl-dev pkg-config
```

### Android NDK linking errors

Ensure `ANDROID_NDK_HOME` is set correctly:
```bash
echo $ANDROID_NDK_HOME
ls $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/*/bin/
```

### "cannot find lipo" (macOS)

Install Xcode Command Line Tools:
```bash
xcode-select --install
```

### Binary size too large

The release profile is already optimized for size. For even smaller binaries, use:
```bash
cargo build --release
strip target/release/libnexus_core.*  # Remove additional symbols
```

Or use UPX compression (use with caution):
```bash
upx --best --lzma target/release/libnexus_core.*
```

## Getting Help

- **Documentation**: Run `cargo doc --open` for API documentation
- **Issues**: [GitHub Issues](https://github.com/kodfikirsanat/focussuite/issues)
- **Examples**: See `examples/` directory
- **Migration Guide**: [docs/DATABASE_MIGRATIONS.md](docs/DATABASE_MIGRATIONS.md)

## Next Steps

After installation:

1. Read the [README.md](README.md) for API overview
2. Check [CHANGELOG.md](CHANGELOG.md) for version history
3. Review [docs/](docs/) for detailed guides
4. Explore [examples/](examples/) for usage patterns
5. Review the [API documentation](https://docs.rs/nexus-core)
