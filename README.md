# UPI

[![Docs](https://img.shields.io/badge/docs-nisoku.org%2FUPI-blue?style=flat)](https://nisoku.org/UPI)
[![CI](https://github.com/Nisoku/UPI/actions/workflows/full.yml/badge.svg)](https://github.com/Nisoku/UPI/actions/workflows/full.yml)

Universal Package Installer.

One command to install anything, anywhere:

> **Documentation:** [nisoku.org/UPI](https://nisoku.org/UPI)

```sh
upi install [...]
```

<p align="center">
  <img src="Docs/assets/images/upi-demo.gif" width="720" alt="UPI demo">
</p>

UPI is a cross-OS macro installer. It translates a generic package name into the correct native install command for your system. It is not a package manager, rather a translator over existing package managers.

## Quick Start

**Unix (Linux / macOS):**

```sh
curl -sfL https://raw.githubusercontent.com/Nisoku/UPI/main/install.sh | sh
```

**Windows (PowerShell):**

```pwsh
iwr https://raw.githubusercontent.com/Nisoku/UPI/main/install.ps1 | iex
```

**Via Cargo (any platform with Rust):**

```sh
cargo install upi
```

## Supported Platforms

| OS      | Manager                   |
|---------|---------------------------|
| macOS   | Homebrew, MacPorts        |
| Debian  | apt                       |
| Fedora  | dnf                       |
| Arch    | pacman                    |
| Windows | winget, chocolatey, scoop |

## How Resolution Works (in order of priority)

1. **Alias** - shorthand maps (`rg` -> `ripgrep`, `py` -> `python`, `nvim` -> `neovim`).
2. **Repology** - global project-to-package-name resolution.
3. **Database lookup** - compressed SQLite seed DB with common packages across all platforms. Supports aliases (`python3` -> `python`) and provenance tracking.
4. **Fallback search** - native package manager regex search (skipped for short queries <= 3 chars).

If no confident match is found, UPI shows "Did you mean: ..." suggestions and exits with an error. Use `--allow-identity` to install the raw query as-is.

The DB is cached at `~/.upi/db/seed.db` and only rehydrated when the seed version changes.

## Repository Layout

```txt
UPI/
  Cargo.toml                    # workspace root
  Build/
    crates/
      upi-core/                 # resolver, DB, OS detection, exec, YAML loader
        data/                   # SQL schema, seed data, version
        platform/               # YAML OS definitions (SSOT)
      upi-cli/                  # thin CLI wrapper (clap)
      upi-net/                  # network logic (ureq, isolated for WASM)
    tests/                      # all tests
    tools/
      db-update/                # seed DB auto-generator from Repology
  debug/
    GUIDE.md                    # architecture and conventions
```

## Regenerating the Seed DB

```sh
cargo run -p db-update
```

## License

Apache 2.0. See [LICENSE](LICENSE).
