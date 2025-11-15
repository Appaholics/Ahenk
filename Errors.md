# Those errors are from CI processess.

## Run Clippy
warning: glib-sys@0.18.1: 
error: failed to run custom build command for `glib-sys v0.18.1`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

Caused by:
  process didn't exit successfully: `/home/runner/work/ahenk/ahenk/target/debug/build/glib-sys-9feb41a102c3dedf/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 'glib-2.0 >= 2.70'

  The system library `glib-2.0` required by crate `glib-sys` was not found.
  The file `glib-2.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.

  HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `glib-2.0.pc`.

warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.

## Build Documentation
warning: glib-sys@0.18.1: 
error: failed to run custom build command for `glib-sys v0.18.1`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

Caused by:
  process didn't exit successfully: `/home/runner/work/ahenk/ahenk/target/debug/build/glib-sys-9feb41a102c3dedf/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 'glib-2.0 >= 2.70'

  The system library `glib-2.0` required by crate `glib-sys` was not found.
  The file `glib-2.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.

  HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `glib-2.0.pc`.

warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.

## Code Coverage
warning: glib-sys@0.18.1: 
error: failed to run custom build command for `glib-sys v0.18.1`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_TEST_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

Caused by:
  process didn't exit successfully: `/home/runner/work/ahenk/ahenk/target/debug/build/glib-sys-d4a34de9dd80cf99/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 'glib-2.0 >= 2.70'

  The system library `glib-2.0` required by crate `glib-sys` was not found.
  The file `glib-2.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.

  HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `glib-2.0.pc`.

warning: build failed, waiting for other jobs to finish...
2025-11-12T11:20:07.294309Z ERROR cargo_tarpaulin: Cargo failed to run! Error: cargo run failed
Error: "Cargo failed to run! Error: cargo run failed"
Error: Process completed with exit code 1.

## Security Audit
Run cargo audit
    Updating crates.io index
     Locking 796 packages to latest Rust 1.93.0-nightly compatible versions
      Adding colored v2.2.0 (available: v3.0.0)
      Adding dirs v5.0.1 (available: v6.0.0)
      Adding indicatif v0.17.11 (available: v0.18.3)
      Adding libp2p v0.53.2 (available: v0.56.0)
      Adding proc-macro-crate v2.0.0 (available: v2.0.2)
      Adding sysinfo v0.30.13 (available: v0.37.2)
      Adding toml v0.8.23 (available: v0.9.8)
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 866 security advisories (from /home/runner/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (797 crate dependencies)
Crate:     ring
Version:   0.16.20
Title:     Some AES functions may panic when overflow checking is enabled.
Date:      2025-03-06
ID:        RUSTSEC-2025-0009
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0009
Solution:  Upgrade to >=0.17.12
Dependency tree:
ring 0.16.20
├── rcgen 0.11.3
│   ├── libp2p-tls 0.4.1
│   │   └── libp2p-quic 0.10.3
│   │       └── libp2p 0.53.2
│   │           └── ahenk 0.1.0
│   └── libp2p-tls 0.3.0
│       └── libp2p 0.53.2
└── libp2p-tls 0.3.0

Crate:     async-std
Version:   1.13.2
Warning:   unmaintained
Title:     async-std has been discontinued
Date:      2025-08-24
ID:        RUSTSEC-2025-0052
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0052
Dependency tree:
async-std 1.13.2
├── quinn 0.11.9
│   └── libp2p-quic 0.10.3
│       └── libp2p 0.53.2
│           └── ahenk 0.1.0
├── libp2p-swarm 0.44.2
│   ├── libp2p-upnp 0.2.2
│   │   └── libp2p 0.53.2
│   ├── libp2p-request-response 0.26.3
│   │   ├── libp2p-rendezvous 0.14.0
│   │   │   └── libp2p 0.53.2
│   │   ├── libp2p-autonat 0.12.0
│   │   │   └── libp2p 0.53.2
│   │   └── libp2p 0.53.2
│   ├── libp2p-rendezvous 0.14.0
│   ├── libp2p-relay 0.17.2
│   │   ├── libp2p-metrics 0.14.1
│   │   │   └── libp2p 0.53.2
│   │   └── libp2p 0.53.2
│   ├── libp2p-ping 0.44.1
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-metrics 0.14.1
│   ├── libp2p-memory-connection-limits 0.2.0
│   │   └── libp2p 0.53.2
│   ├── libp2p-mdns 0.45.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-kad 0.45.3
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-identify 0.44.2
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-gossipsub 0.46.1
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-floodsub 0.44.0
│   │   └── libp2p 0.53.2
│   ├── libp2p-dcutr 0.11.0
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-connection-limits 0.3.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-autonat 0.12.0
│   ├── libp2p-allow-block-list 0.3.0
│   │   └── libp2p 0.53.2
│   └── libp2p 0.53.2
├── libp2p-quic 0.10.3
├── libp2p-mdns 0.45.1
├── ahenk 0.1.0
└── async-std-resolver 0.24.4
    └── libp2p-dns 0.41.1
        └── libp2p 0.53.2

Crate:     atk
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0413
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0413
Dependency tree:
atk 0.18.2
└── gtk 0.18.2
    ├── wry 0.53.5
    │   └── tauri-runtime-wry 2.9.1
    │       └── tauri 2.9.2
    │           └── ahenk 0.1.0
    ├── webkit2gtk 2.0.1
    │   ├── wry 0.53.5
    │   ├── tauri-runtime-wry 2.9.1
    │   ├── tauri-runtime 2.9.1
    │   │   ├── tauri-runtime-wry 2.9.1
    │   │   └── tauri 2.9.2
    │   └── tauri 2.9.2
    ├── tauri-runtime-wry 2.9.1
    ├── tauri-runtime 2.9.1
    ├── tauri 2.9.2
    ├── tao 0.34.5
    │   └── tauri-runtime-wry 2.9.1
    ├── muda 0.17.1
    │   ├── tray-icon 0.21.2
    │   │   └── tauri 2.9.2
    │   └── tauri 2.9.2
    └── libappindicator 0.9.0
        └── tray-icon 0.21.2

Crate:     atk-sys
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0416
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0416
Dependency tree:
atk-sys 0.18.2
├── gtk-sys 0.18.2
│   ├── webkit2gtk-sys 2.0.1
│   │   ├── wry 0.53.5
│   │   │   └── tauri-runtime-wry 2.9.1
│   │   │       └── tauri 2.9.2
│   │   │           └── ahenk 0.1.0
│   │   └── webkit2gtk 2.0.1
│   │       ├── wry 0.53.5
│   │       ├── tauri-runtime-wry 2.9.1
│   │       ├── tauri-runtime 2.9.1
│   │       │   ├── tauri-runtime-wry 2.9.1
│   │       │   └── tauri 2.9.2
│   │       └── tauri 2.9.2
│   ├── webkit2gtk 2.0.1
│   ├── libappindicator-sys 0.9.0
│   │   └── libappindicator 0.9.0
│   │       └── tray-icon 0.21.2
│   │           └── tauri 2.9.2
│   ├── libappindicator 0.9.0
│   └── gtk 0.18.2
│       ├── wry 0.53.5
│       ├── webkit2gtk 2.0.1
│       ├── tauri-runtime-wry 2.9.1
│       ├── tauri-runtime 2.9.1
│       ├── tauri 2.9.2
│       ├── tao 0.34.5
│       │   └── tauri-runtime-wry 2.9.1
│       ├── muda 0.17.1
│       │   ├── tray-icon 0.21.2
│       │   └── tauri 2.9.2
│       └── libappindicator 0.9.0
└── atk 0.18.2
    └── gtk 0.18.2

Crate:     daemonize
Version:   0.5.0
Warning:   unmaintained
Title:     `daemonize` is Unmaintained
Date:      2025-09-14
ID:        RUSTSEC-2025-0069
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0069
Dependency tree:
daemonize 0.5.0
└── ahenk 0.1.0

Crate:     fxhash
Version:   0.2.1
Warning:   unmaintained
Title:     fxhash - no longer maintained
Date:      2025-09-05
ID:        RUSTSEC-2025-0057
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0057
Dependency tree:
fxhash 0.2.1
└── selectors 0.24.0
    └── kuchikiki 0.8.8-speedreader
        ├── wry 0.53.5
        │   └── tauri-runtime-wry 2.9.1
        │       └── tauri 2.9.2
        │           └── ahenk 0.1.0
        └── tauri-utils 2.8.0
            ├── tauri-runtime-wry 2.9.1
            ├── tauri-runtime 2.9.1
            │   ├── tauri-runtime-wry 2.9.1
            │   └── tauri 2.9.2
            ├── tauri-macros 2.5.0
            │   └── tauri 2.9.2
            ├── tauri-codegen 2.5.0
            │   └── tauri-macros 2.5.0
            ├── tauri-build 2.5.1
            │   ├── tauri 2.9.2
            │   └── ahenk 0.1.0
            └── tauri 2.9.2

Crate:     gdk
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0412
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0412
Dependency tree:
gdk 0.18.2
├── webkit2gtk 2.0.1
│   ├── wry 0.53.5
│   │   └── tauri-runtime-wry 2.9.1
│   │       └── tauri 2.9.2
│   │           └── ahenk 0.1.0
│   ├── tauri-runtime-wry 2.9.1
│   ├── tauri-runtime 2.9.1
│   │   ├── tauri-runtime-wry 2.9.1
│   │   └── tauri 2.9.2
│   └── tauri 2.9.2
├── gtk 0.18.2
│   ├── wry 0.53.5
│   ├── webkit2gtk 2.0.1
│   ├── tauri-runtime-wry 2.9.1
│   ├── tauri-runtime 2.9.1
│   ├── tauri 2.9.2
│   ├── tao 0.34.5
│   │   └── tauri-runtime-wry 2.9.1
│   ├── muda 0.17.1
│   │   ├── tray-icon 0.21.2
│   │   │   └── tauri 2.9.2
│   │   └── tauri 2.9.2
│   └── libappindicator 0.9.0
│       └── tray-icon 0.21.2
└── gdkx11 0.18.2
    └── wry 0.53.5

Crate:     gdk-sys
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0418
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0418
Dependency tree:
gdk-sys 0.18.2
├── webkit2gtk-sys 2.0.1
│   ├── wry 0.53.5
│   │   └── tauri-runtime-wry 2.9.1
│   │       └── tauri 2.9.2
│   │           └── ahenk 0.1.0
│   └── webkit2gtk 2.0.1
│       ├── wry 0.53.5
│       ├── tauri-runtime-wry 2.9.1
│       ├── tauri-runtime 2.9.1
│       │   ├── tauri-runtime-wry 2.9.1
│       │   └── tauri 2.9.2
│       └── tauri 2.9.2
├── webkit2gtk 2.0.1
├── gtk-sys 0.18.2
│   ├── webkit2gtk-sys 2.0.1
│   ├── webkit2gtk 2.0.1
│   ├── libappindicator-sys 0.9.0
│   │   └── libappindicator 0.9.0
│   │       └── tray-icon 0.21.2
│   │           └── tauri 2.9.2
│   ├── libappindicator 0.9.0
│   └── gtk 0.18.2
│       ├── wry 0.53.5
│       ├── webkit2gtk 2.0.1
│       ├── tauri-runtime-wry 2.9.1
│       ├── tauri-runtime 2.9.1
│       ├── tauri 2.9.2
│       ├── tao 0.34.5
│       │   └── tauri-runtime-wry 2.9.1
│       ├── muda 0.17.1
│       │   ├── tray-icon 0.21.2
│       │   └── tauri 2.9.2
│       └── libappindicator 0.9.0
├── gdkx11-sys 0.18.2
│   ├── tao 0.34.5
│   └── gdkx11 0.18.2
│       └── wry 0.53.5
├── gdkwayland-sys 0.18.2
│   └── tao 0.34.5
└── gdk 0.18.2
    ├── webkit2gtk 2.0.1
    ├── gtk 0.18.2
    └── gdkx11 0.18.2

Crate:     gdkwayland-sys
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0411
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0411
Dependency tree:
gdkwayland-sys 0.18.2
└── tao 0.34.5
    └── tauri-runtime-wry 2.9.1
        └── tauri 2.9.2
            └── ahenk 0.1.0

Crate:     gdkx11
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0417
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0417
Dependency tree:
gdkx11 0.18.2
└── wry 0.53.5
    └── tauri-runtime-wry 2.9.1
        └── tauri 2.9.2
            └── ahenk 0.1.0

Crate:     gdkx11-sys
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0414
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0414
Dependency tree:
gdkx11-sys 0.18.2
├── tao 0.34.5
│   └── tauri-runtime-wry 2.9.1
│       └── tauri 2.9.2
│           └── ahenk 0.1.0
└── gdkx11 0.18.2
    └── wry 0.53.5
        └── tauri-runtime-wry 2.9.1

Crate:     gtk
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0415
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0415
Dependency tree:
gtk 0.18.2
├── wry 0.53.5
│   └── tauri-runtime-wry 2.9.1
│       └── tauri 2.9.2
│           └── ahenk 0.1.0
├── webkit2gtk 2.0.1
│   ├── wry 0.53.5
│   ├── tauri-runtime-wry 2.9.1
│   ├── tauri-runtime 2.9.1
│   │   ├── tauri-runtime-wry 2.9.1
│   │   └── tauri 2.9.2
│   └── tauri 2.9.2
├── tauri-runtime-wry 2.9.1
├── tauri-runtime 2.9.1
├── tauri 2.9.2
├── tao 0.34.5
│   └── tauri-runtime-wry 2.9.1
├── muda 0.17.1
│   ├── tray-icon 0.21.2
│   │   └── tauri 2.9.2
│   └── tauri 2.9.2
└── libappindicator 0.9.0
    └── tray-icon 0.21.2

Crate:     gtk-sys
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0420
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0420
Dependency tree:
gtk-sys 0.18.2
├── webkit2gtk-sys 2.0.1
│   ├── wry 0.53.5
│   │   └── tauri-runtime-wry 2.9.1
│   │       └── tauri 2.9.2
│   │           └── ahenk 0.1.0
│   └── webkit2gtk 2.0.1
│       ├── wry 0.53.5
│       ├── tauri-runtime-wry 2.9.1
│       ├── tauri-runtime 2.9.1
│       │   ├── tauri-runtime-wry 2.9.1
│       │   └── tauri 2.9.2
│       └── tauri 2.9.2
├── webkit2gtk 2.0.1
├── libappindicator-sys 0.9.0
│   └── libappindicator 0.9.0
│       └── tray-icon 0.21.2
│           └── tauri 2.9.2
├── libappindicator 0.9.0
└── gtk 0.18.2
    ├── wry 0.53.5
    ├── webkit2gtk 2.0.1
    ├── tauri-runtime-wry 2.9.1
    ├── tauri-runtime 2.9.1
    ├── tauri 2.9.2
    ├── tao 0.34.5
    │   └── tauri-runtime-wry 2.9.1
    ├── muda 0.17.1
    │   ├── tray-icon 0.21.2
    │   └── tauri 2.9.2
    └── libappindicator 0.9.0

Crate:     gtk3-macros
Version:   0.18.2
Warning:   unmaintained
Title:     gtk-rs GTK3 bindings - no longer maintained
Date:      2024-03-04
ID:        RUSTSEC-2024-0419
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0419
Dependency tree:
gtk3-macros 0.18.2
└── gtk 0.18.2
    ├── wry 0.53.5
    │   └── tauri-runtime-wry 2.9.1
    │       └── tauri 2.9.2
    │           └── ahenk 0.1.0
    ├── webkit2gtk 2.0.1
    │   ├── wry 0.53.5
    │   ├── tauri-runtime-wry 2.9.1
    │   ├── tauri-runtime 2.9.1
    │   │   ├── tauri-runtime-wry 2.9.1
    │   │   └── tauri 2.9.2
    │   └── tauri 2.9.2
    ├── tauri-runtime-wry 2.9.1
    ├── tauri-runtime 2.9.1
    ├── tauri 2.9.2
    ├── tao 0.34.5
    │   └── tauri-runtime-wry 2.9.1
    ├── muda 0.17.1
    │   ├── tray-icon 0.21.2
    │   │   └── tauri 2.9.2
    │   └── tauri 2.9.2
    └── libappindicator 0.9.0
        └── tray-icon 0.21.2

Crate:     instant
Version:   0.1.13
Warning:   unmaintained
Title:     `instant` is unmaintained
Date:      2024-09-01
ID:        RUSTSEC-2024-0384
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0384
Dependency tree:
instant 0.1.13
├── libp2p-swarm 0.44.2
│   ├── libp2p-upnp 0.2.2
│   │   └── libp2p 0.53.2
│   │       └── ahenk 0.1.0
│   ├── libp2p-request-response 0.26.3
│   │   ├── libp2p-rendezvous 0.14.0
│   │   │   └── libp2p 0.53.2
│   │   ├── libp2p-autonat 0.12.0
│   │   │   └── libp2p 0.53.2
│   │   └── libp2p 0.53.2
│   ├── libp2p-rendezvous 0.14.0
│   ├── libp2p-relay 0.17.2
│   │   ├── libp2p-metrics 0.14.1
│   │   │   └── libp2p 0.53.2
│   │   └── libp2p 0.53.2
│   ├── libp2p-ping 0.44.1
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-metrics 0.14.1
│   ├── libp2p-memory-connection-limits 0.2.0
│   │   └── libp2p 0.53.2
│   ├── libp2p-mdns 0.45.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-kad 0.45.3
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-identify 0.44.2
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-gossipsub 0.46.1
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-floodsub 0.44.0
│   │   └── libp2p 0.53.2
│   ├── libp2p-dcutr 0.11.0
│   │   ├── libp2p-metrics 0.14.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-connection-limits 0.3.1
│   │   └── libp2p 0.53.2
│   ├── libp2p-autonat 0.12.0
error: 1 vulnerability found!
warning: 23 allowed warnings found
│   ├── libp2p-allow-block-list 0.3.0
│   │   └── libp2p 0.53.2
│   └── libp2p 0.53.2
├── libp2p-request-response 0.26.3
├── libp2p-rendezvous 0.14.0
├── libp2p-ping 0.44.1
├── libp2p-metrics 0.14.1
├── libp2p-kad 0.45.3
├── libp2p-gossipsub 0.46.1
├── libp2p-dcutr 0.11.0
├── libp2p-autonat 0.12.0
├── libp2p 0.53.2
├── futures-ticker 0.0.3
│   └── libp2p-gossipsub 0.46.1
└── fastrand 1.9.0
    └── futures-lite 1.13.0
        └── async-io 1.13.0
            └── libp2p-tcp 0.41.0
                └── libp2p 0.53.2

Crate:     paste
Version:   1.0.15
Warning:   unmaintained
Title:     paste - no longer maintained
Date:      2024-10-07
ID:        RUSTSEC-2024-0436
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0436
Dependency tree:
paste 1.0.15
└── netlink-packet-utils 0.5.2
    ├── rtnetlink 0.13.1
    │   └── if-watch 3.2.1
    │       ├── libp2p-tcp 0.41.0
    │       │   └── libp2p 0.53.2
    │       │       └── ahenk 0.1.0
    │       ├── libp2p-quic 0.10.3
    │       │   └── libp2p 0.53.2
    │       └── libp2p-mdns 0.45.1
    │           └── libp2p 0.53.2
    ├── netlink-packet-route 0.17.1
    │   ├── rtnetlink 0.13.1
    │   └── if-watch 3.2.1
    └── netlink-packet-core 0.7.0
        ├── rtnetlink 0.13.1
        ├── netlink-proto 0.11.5
        │   ├── rtnetlink 0.13.1
        │   └── if-watch 3.2.1
        ├── netlink-packet-route 0.17.1
        └── if-watch 3.2.1

Crate:     proc-macro-error
Version:   1.0.4
Warning:   unmaintained
Title:     proc-macro-error is unmaintained
Date:      2024-09-01
ID:        RUSTSEC-2024-0370
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0370
Dependency tree:
proc-macro-error 1.0.4
├── gtk3-macros 0.18.2
│   └── gtk 0.18.2
│       ├── wry 0.53.5
│       │   └── tauri-runtime-wry 2.9.1
│       │       └── tauri 2.9.2
│       │           └── ahenk 0.1.0
│       ├── webkit2gtk 2.0.1
│       │   ├── wry 0.53.5
│       │   ├── tauri-runtime-wry 2.9.1
│       │   ├── tauri-runtime 2.9.1
│       │   │   ├── tauri-runtime-wry 2.9.1
│       │   │   └── tauri 2.9.2
│       │   └── tauri 2.9.2
│       ├── tauri-runtime-wry 2.9.1
│       ├── tauri-runtime 2.9.1
│       ├── tauri 2.9.2
│       ├── tao 0.34.5
│       │   └── tauri-runtime-wry 2.9.1
│       ├── muda 0.17.1
│       │   ├── tray-icon 0.21.2
│       │   │   └── tauri 2.9.2
│       │   └── tauri 2.9.2
│       └── libappindicator 0.9.0
│           └── tray-icon 0.21.2
└── glib-macros 0.18.5
    └── glib 0.18.5
        ├── webkit2gtk 2.0.1
        ├── soup3 0.5.0
        │   ├── wry 0.53.5
        │   └── webkit2gtk 2.0.1
        ├── pango 0.18.3
        │   ├── gtk 0.18.2
        │   └── gdk 0.18.2
        │       ├── webkit2gtk 2.0.1
        │       ├── gtk 0.18.2
        │       └── gdkx11 0.18.2
        │           └── wry 0.53.5
        ├── libappindicator 0.9.0
        ├── javascriptcore-rs 1.1.2
        │   ├── wry 0.53.5
        │   └── webkit2gtk 2.0.1
        ├── gtk 0.18.2
        ├── gio 0.18.4
        │   ├── webkit2gtk 2.0.1
        │   ├── soup3 0.5.0
        │   ├── pango 0.18.3
        │   ├── gtk 0.18.2
        │   ├── gdkx11 0.18.2
        │   ├── gdk-pixbuf 0.18.5
        │   │   ├── gtk 0.18.2
        │   │   └── gdk 0.18.2
        │   └── gdk 0.18.2
        ├── gdkx11 0.18.2
        ├── gdk-pixbuf 0.18.5
        ├── gdk 0.18.2
        ├── cairo-rs 0.18.5
        │   ├── webkit2gtk 2.0.1
        │   ├── gtk 0.18.2
        │   └── gdk 0.18.2
        └── atk 0.18.2
            └── gtk 0.18.2

Crate:     ring
Version:   0.16.20
Warning:   unmaintained
Title:     Versions of *ring* prior to 0.17 are unmaintained.
Date:      2025-03-05
ID:        RUSTSEC-2025-0010
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0010

Crate:     unic-char-property
Version:   0.9.0
Warning:   unmaintained
Title:     `unic-char-property` is unmaintained
Date:      2025-10-18
ID:        RUSTSEC-2025-0081
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0081
Dependency tree:
unic-char-property 0.9.0
└── unic-ucd-ident 0.9.0
    └── urlpattern 0.3.0
        └── tauri-utils 2.8.0
            ├── tauri-runtime-wry 2.9.1
            │   └── tauri 2.9.2
            │       └── ahenk 0.1.0
            ├── tauri-runtime 2.9.1
            │   ├── tauri-runtime-wry 2.9.1
            │   └── tauri 2.9.2
            ├── tauri-macros 2.5.0
            │   └── tauri 2.9.2
            ├── tauri-codegen 2.5.0
            │   └── tauri-macros 2.5.0
            ├── tauri-build 2.5.1
            │   ├── tauri 2.9.2
            │   └── ahenk 0.1.0
            └── tauri 2.9.2

Crate:     unic-char-range
Version:   0.9.0
Warning:   unmaintained
Title:     `unic-char-range` is unmaintained
Date:      2025-10-18
ID:        RUSTSEC-2025-0075
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0075
Dependency tree:
unic-char-range 0.9.0
├── unic-ucd-ident 0.9.0
│   └── urlpattern 0.3.0
│       └── tauri-utils 2.8.0
│           ├── tauri-runtime-wry 2.9.1
│           │   └── tauri 2.9.2
│           │       └── ahenk 0.1.0
│           ├── tauri-runtime 2.9.1
│           │   ├── tauri-runtime-wry 2.9.1
│           │   └── tauri 2.9.2
│           ├── tauri-macros 2.5.0
│           │   └── tauri 2.9.2
│           ├── tauri-codegen 2.5.0
│           │   └── tauri-macros 2.5.0
│           ├── tauri-build 2.5.1
│           │   ├── tauri 2.9.2
│           │   └── ahenk 0.1.0
│           └── tauri 2.9.2
└── unic-char-property 0.9.0
    └── unic-ucd-ident 0.9.0

Crate:     unic-common
Version:   0.9.0
Warning:   unmaintained
Title:     `unic-common` is unmaintained
Date:      2025-10-18
ID:        RUSTSEC-2025-0080
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0080
Dependency tree:
unic-common 0.9.0
└── unic-ucd-version 0.9.0
    └── unic-ucd-ident 0.9.0
        └── urlpattern 0.3.0
            └── tauri-utils 2.8.0
                ├── tauri-runtime-wry 2.9.1
                │   └── tauri 2.9.2
                │       └── ahenk 0.1.0
                ├── tauri-runtime 2.9.1
                │   ├── tauri-runtime-wry 2.9.1
                │   └── tauri 2.9.2
                ├── tauri-macros 2.5.0
                │   └── tauri 2.9.2
                ├── tauri-codegen 2.5.0
                │   └── tauri-macros 2.5.0
                ├── tauri-build 2.5.1
                │   ├── tauri 2.9.2
                │   └── ahenk 0.1.0
                └── tauri 2.9.2

Crate:     unic-ucd-ident
Version:   0.9.0
Warning:   unmaintained
Title:     `unic-ucd-ident` is unmaintained
Date:      2025-10-18
ID:        RUSTSEC-2025-0100
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0100
Dependency tree:
unic-ucd-ident 0.9.0
└── urlpattern 0.3.0
    └── tauri-utils 2.8.0
        ├── tauri-runtime-wry 2.9.1
        │   └── tauri 2.9.2
        │       └── ahenk 0.1.0
        ├── tauri-runtime 2.9.1
        │   ├── tauri-runtime-wry 2.9.1
        │   └── tauri 2.9.2
        ├── tauri-macros 2.5.0
        │   └── tauri 2.9.2
        ├── tauri-codegen 2.5.0
        │   └── tauri-macros 2.5.0
        ├── tauri-build 2.5.1
        │   ├── tauri 2.9.2
        │   └── ahenk 0.1.0
        └── tauri 2.9.2

Crate:     unic-ucd-version
Version:   0.9.0
Warning:   unmaintained
Title:     `unic-ucd-version` is unmaintained
Date:      2025-10-18
ID:        RUSTSEC-2025-0098
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0098
Dependency tree:
unic-ucd-version 0.9.0
└── unic-ucd-ident 0.9.0
    └── urlpattern 0.3.0
        └── tauri-utils 2.8.0
            ├── tauri-runtime-wry 2.9.1
            │   └── tauri 2.9.2
            │       └── ahenk 0.1.0
            ├── tauri-runtime 2.9.1
            │   ├── tauri-runtime-wry 2.9.1
            │   └── tauri 2.9.2
            ├── tauri-macros 2.5.0
            │   └── tauri 2.9.2
            ├── tauri-codegen 2.5.0
            │   └── tauri-macros 2.5.0
            ├── tauri-build 2.5.1
            │   ├── tauri 2.9.2
            │   └── ahenk 0.1.0
            └── tauri 2.9.2

Crate:     glib
Version:   0.18.5
Warning:   unsound
Title:     Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
Date:      2024-03-30
ID:        RUSTSEC-2024-0429
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0429
Dependency tree:
glib 0.18.5
├── webkit2gtk 2.0.1
│   ├── wry 0.53.5
│   │   └── tauri-runtime-wry 2.9.1
│   │       └── tauri 2.9.2
│   │           └── ahenk 0.1.0
│   ├── tauri-runtime-wry 2.9.1
│   ├── tauri-runtime 2.9.1
│   │   ├── tauri-runtime-wry 2.9.1
│   │   └── tauri 2.9.2
│   └── tauri 2.9.2
├── soup3 0.5.0
│   ├── wry 0.53.5
│   └── webkit2gtk 2.0.1
├── pango 0.18.3
│   ├── gtk 0.18.2
│   │   ├── wry 0.53.5
│   │   ├── webkit2gtk 2.0.1
│   │   ├── tauri-runtime-wry 2.9.1
│   │   ├── tauri-runtime 2.9.1
│   │   ├── tauri 2.9.2
│   │   ├── tao 0.34.5
│   │   │   └── tauri-runtime-wry 2.9.1
│   │   ├── muda 0.17.1
│   │   │   ├── tray-icon 0.21.2
│   │   │   │   └── tauri 2.9.2
│   │   │   └── tauri 2.9.2
│   │   └── libappindicator 0.9.0
│   │       └── tray-icon 0.21.2
│   └── gdk 0.18.2
│       ├── webkit2gtk 2.0.1
│       ├── gtk 0.18.2
│       └── gdkx11 0.18.2
│           └── wry 0.53.5
├── libappindicator 0.9.0
├── javascriptcore-rs 1.1.2
│   ├── wry 0.53.5
│   └── webkit2gtk 2.0.1
├── gtk 0.18.2
├── gio 0.18.4
│   ├── webkit2gtk 2.0.1
│   ├── soup3 0.5.0
│   ├── pango 0.18.3
│   ├── gtk 0.18.2
│   ├── gdkx11 0.18.2
│   ├── gdk-pixbuf 0.18.5
│   │   ├── gtk 0.18.2
│   │   └── gdk 0.18.2
│   └── gdk 0.18.2
├── gdkx11 0.18.2
├── gdk-pixbuf 0.18.5
├── gdk 0.18.2
├── cairo-rs 0.18.5
│   ├── webkit2gtk 2.0.1
│   ├── gtk 0.18.2
│   └── gdk 0.18.2
└── atk 0.18.2
    └── gtk 0.18.2

Error: Process completed with exit code 1.

## Test Suite (ubuntu-latest nightly)

warning: glib-sys@0.18.1: 
error: failed to run custom build command for `glib-sys v0.18.1`
note: To improve backtraces for build dependencies, set the CARGO_PROFILE_TEST_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

Caused by:
  process didn't exit successfully: `/home/runner/work/ahenk/ahenk/target/debug/build/glib-sys-25a6fb1fcf3a1a23/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=GLIB_2.0_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:warning=
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags glib-2.0 'glib-2.0 >= 2.70'

  The system library `glib-2.0` required by crate `glib-sys` was not found.
  The file `glib-2.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.

  HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `glib-2.0.pc`.

warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.

## Cross-Platform Build Check (aarch64-apple-darwin)
 warning: ring@0.17.14: cc: error: unrecognized debug output level 'full'
warning: ring@0.17.14: cc: error: unrecognized command-line option '-arch'
warning: ring@0.17.14: cc: error: unrecognized command-line option '-mmacosx-version-min=11.0'
error: failed to run custom build command for `ring v0.17.14`

Caused by:
  process didn't exit successfully: `/home/runner/work/ahenk/ahenk/target/debug/build/ring-01dd99b48d541217/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR
  cargo:rerun-if-env-changed=CARGO_PKG_NAME
  cargo:rerun-if-env-changed=CARGO_PKG_VERSION_MAJOR
  cargo:rerun-if-env-changed=CARGO_PKG_VERSION_MINOR
  cargo:rerun-if-env-changed=CARGO_PKG_VERSION_PATCH
  cargo:rerun-if-env-changed=CARGO_PKG_VERSION_PRE
  cargo:rerun-if-env-changed=CARGO_MANIFEST_LINKS
  cargo:rerun-if-env-changed=RING_PREGENERATE_ASM
  cargo:rerun-if-env-changed=OUT_DIR
  cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH
  cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS
  cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ENV
  cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ENDIAN
  OPT_LEVEL = Some(0)
  OUT_DIR = Some(/home/runner/work/ahenk/ahenk/target/aarch64-apple-darwin/debug/build/ring-66ee57531ed8ab40/out)
  TARGET = Some(aarch64-apple-darwin)
  CARGO_ENCODED_RUSTFLAGS = Some()
  HOST = Some(x86_64-unknown-linux-gnu)
  cargo:rerun-if-env-changed=CC_aarch64-apple-darwin
  CC_aarch64-apple-darwin = None
  cargo:rerun-if-env-changed=CC_aarch64_apple_darwin
  CC_aarch64_apple_darwin = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = None
  cargo:rerun-if-env-changed=CROSS_COMPILE
  CROSS_COMPILE = None
  RUSTC_LINKER = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  RUSTC_WRAPPER = None
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some(true)
  CARGO_CFG_TARGET_FEATURE = Some(aes,crc,dit,dotprod,dpb,dpb2,fcma,fhm,flagm,flagm2,fp16,frintts,jsconv,lor,lse,lse2,neon,paca,pacg,pan,pmuv3,ras,rcpc,rcpc2,rdm,sb,sha2,sha3,ssbs,v8.1a,v8.2a,v8.3a,v8.4a,vh)
  cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET
  MACOSX_DEPLOYMENT_TARGET = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS_aarch64_apple_darwin
  CFLAGS_aarch64_apple_darwin = None
  cargo:rerun-if-env-changed=CFLAGS_aarch64-apple-darwin
  CFLAGS_aarch64-apple-darwin = None
  cargo:warning=cc: error: unrecognized debug output level 'full'
  cargo:warning=cc: error: unrecognized command-line option '-arch'
  cargo:warning=cc: error: unrecognized command-line option '-mmacosx-version-min=11.0'

  --- stderr


  error occurred in cc-rs: command did not execute successfully (status code exit status: 1): LC_ALL="C" "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-gdwarf-2" "-fno-omit-frame-pointer" "-arch" "arm64" "-mmacosx-version-min=11.0" "-I" "/home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.17.14/include" "-I" "/home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.17.14/pregenerated" "-Wall" "-Wextra" "-fvisibility=hidden" "-std=c1x" "-Wall" "-Wbad-function-cast" "-Wcast-align" "-Wcast-qual" "-Wconversion" "-Wmissing-field-initializers" "-Wmissing-include-dirs" "-Wnested-externs" "-Wredundant-decls" "-Wshadow" "-Wsign-compare" "-Wsign-conversion" "-Wstrict-prototypes" "-Wundef" "-Wuninitialized" "-gfull" "-DNDEBUG" "-o" "/home/runner/work/ahenk/ahenk/target/aarch64-apple-darwin/debug/build/ring-66ee57531ed8ab40/out/25ac62e5b3c53843-curve25519.o" "-c" "/home/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.17.14/crypto/curve25519/curve25519.c"


warning: build failed, waiting for other jobs to finish...
Error: Process completed with exit code 101.