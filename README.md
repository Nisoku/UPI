# UPI

Universal Package Installer.

One command to install anything, anywhere:

```sh
upi install [...]
```

UPI is a cross-OS macro installer. It translates a generic package name into the correct native install command for your system. It is not a package manager, rather a translator over existing package managers.

## Quick Start

```sh
cargo install upi
upi install [...]
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
# Edit Build/data/seed-data.sql with new mappings, then:
cargo run -p db-build

# Bump the version in Build/data/seed-version.txt if the schema changes.
```

## License

Apache 2.0. See [LICENSE](LICENSE).
