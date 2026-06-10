---
title: "Installation"
description: "Install UPI on any platform"
---

## Quick Install (Shell)

**Unix (Linux / macOS):**

```bash
curl -sfL https://raw.githubusercontent.com/Nisoku/UPI/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
iwr https://raw.githubusercontent.com/Nisoku/UPI/main/install.ps1 | iex
```

Each script detects your OS and architecture, downloads the correct binary from the latest release, and installs it to `/usr/local/bin` (Unix) or `%USERPROFILE%\.local\bin` (Windows). No dependencies required.

Override the version or destination with environment variables:

Unix:

```bash
UPI_VERSION=v0.1.0 UPI_INSTALL_DIR=~/.local/bin \
  curl -sfL https://raw.githubusercontent.com/Nisoku/UPI/main/install.sh | sh
```

Windows:

```powershell
$env:UPI_VERSION = "v0.1.0"
$env:UPI_INSTALL_DIR = "$env:USERPROFILE\bin"
iwr https://raw.githubusercontent.com/Nisoku/UPI/main/install.ps1 | iex
```

## From Source (crates.io)

```bash
cargo install upi
```

This compiles UPI from source. The binary is placed in `~/.cargo/bin/`.

## Pre-built Binaries

Download the latest binary for your platform from the [releases page](https://github.com/Nisoku/UPI/releases).

Available targets:

| Platform              | Architecture              |
|-----------------------|---------------------------|
| macOS (Intel)         | x86_64-apple-darwin       |
| macOS (Apple Silicon) | aarch64-apple-darwin      |
| Linux                 | x86_64-unknown-linux-gnu  |
| Linux (musl)          | x86_64-unknown-linux-musl |
| Windows               | x86_64-pc-windows-msvc    |

## Build from Source (Git)

```bash
git clone https://github.com/Nisoku/UPI.git
cd UPI
cargo build --release
./target/release/upi --help
```

## Requirements

- **Rust 1.75+** for building from source
- **SQLite**: the bundled `libsqlite3-sys` statically links SQLite, no system library needed
- **Network**: only required for Repology lookups (optional; offline mode works without it)

## First Run

On first run, UPI decompresses the embedded seed database to `~/.upi/db/seed.db`. This happens automatically and requires no user action.

## Next Steps

::: grids
::: grid
::: button "Quick Start" ./quickstart.md icon:play
:::
::: grid
::: button "Core Concepts" ./concepts.md icon:book
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
