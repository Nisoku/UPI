---
title: "Installation"
description: "Install UPI on any platform"
---

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
