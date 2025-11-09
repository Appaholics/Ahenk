.PHONY: help setup-targets clean \
        build-ios build-android build-macos build-linux build-windows build-watchos build-wearos build-all \
        test check

# Default target
help:
	@echo "Focus Suite - nexus-core Build Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup-targets    Install all Rust cross-compilation targets"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build-ios        Build for iOS (device and simulator)"
	@echo "  make build-android    Build for Android (all architectures)"
	@echo "  make build-macos      Build for macOS (universal binary)"
	@echo "  make build-linux      Build for Linux (x64 and ARM64)"
	@echo "  make build-windows    Build for Windows (x64 and ARM64)"
	@echo "  make build-watchos    Build for WatchOS (device and simulator)"
	@echo "  make build-wearos     Build for WearOS (arm64 and arm)"
	@echo "  make build-all        Build for all platforms (host-dependent)"
	@echo ""
	@echo "Development:"
	@echo "  make test             Run Rust tests"
	@echo "  make check            Run cargo check"
	@echo "  make clean            Clean build artifacts"
	@echo ""
	@echo "See docs/cross-compilation.md for detailed information"

setup-targets:
	@echo "Setting up cross-compilation targets..."
	@cd scripts && ./setup-targets.sh

build-ios:
	@echo "Building for iOS..."
	@cd scripts && ./build-ios.sh

build-android:
	@echo "Building for Android..."
	@cd scripts && ./build-android.sh

build-macos:
	@echo "Building for macOS..."
	@cd scripts && ./build-macos.sh

build-linux:
	@echo "Building for Linux..."
	@cd scripts && ./build-linux.sh

build-windows:
	@echo "Building for Windows..."
	@cd scripts && ./build-windows.sh

build-watchos:
	@echo "Building for WatchOS..."
	@cd scripts && ./build-watchos.sh

build-wearos:
	@echo "Building for WearOS..."
	@cd scripts && ./build-wearos.sh

build-all:
	@echo "Building for all platforms..."
	@cd scripts && ./build-all.sh

test:
	@echo "Running tests..."
	@cargo test

check:
	@echo "Running cargo check..."
	@cargo check

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/ios target/android target/macos target/linux target/windows target/watchos target/wearos
