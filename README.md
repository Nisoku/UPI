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

## Repository Layout

```txt
UPI/
  Cargo.toml                # workspace root
  Build/
    crates/
      upi-core/             # resolver, DB, OS detection, Repology
      upi-cli/              # thin CLI wrapper
    platform/               # YAML OS definitions
  Demo/
  Docs/
```

## License

Apache 2.0. See [LICENSE](LICENSE).
