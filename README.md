# UPI

Universal Package Installer.

One command to install anything, anywhere:

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

1. **Repology** - global project-to-package-name resolution.
2. **Database lookup** - compressed SQLite seed DB with common packages across all platforms. Supports aliases (`python3` -> `python`) and provenance tracking.
3. **Fallback search** - native package manager regex search.

The DB is cached at `~/.upi/db/seed.db` and only rehydrated when the seed version changes.

## Repository Layout

```txt
UPI/
  Cargo.toml                    # workspace root
  Build/
    crates/
      upi-core/                 # resolver, DB, OS detection, exec, YAML loader
      upi-cli/                  # thin CLI wrapper (clap)
      upi-net/                  # network logic (ureq, isolated for WASM)
    data/                       # SQL schema, seed data, build tool, version
    platform/                   # YAML OS definitions (SSOT)
    tests/                      # all tests
    tools/
      db-build/                 # Rust helper to generate seed.db.zst
  debug/
    GUIDE.md                    # architecture and conventions
```

## Regenerating the Seed DB

```sh
cargo run -p db-update
```

## License

Apache 2.0. See [LICENSE](LICENSE).
