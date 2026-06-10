---
title: "Supported Platforms"
description: "OS families, package managers, and YAML configuration"
---

UPI currently supports five OS families. Each is configured through a YAML file that defines the package manager, install command, detection rules, and naming conventions.

## Supported OS Families

| OS              | Manager                   | YAML File      |
|-----------------|---------------------------|----------------|
| macOS           | Homebrew                  | `macos.yaml`   |
| Debian / Ubuntu | apt                       | `debian.yaml`  |
| Fedora / RHEL   | dnf                       | `fedora.yaml`  |
| Arch Linux      | pacman                    | `arch.yaml`    |
| Windows         | winget, chocolatey, scoop | `windows.yaml` |

## YAML OS Definitions

OS behavior is defined in `Build/platform/`. Each file specifies:

```yaml
name: macos
manager: homebrew
install: brew install {{package}}
search: brew search {{package}}
provides: brew --prefix {{package}}
```

The YAML schema supports:

- **install**: template for the install command
- **search**: template for the search command
- **provides**: template for checking if a package is installed
- **sudo**: whether the command needs privilege escalation
- **repos**: Repology repository names to match against
- **binary_paths**: paths to check for installed binaries
- **file_extension**: package file extension such as `.apk` or `.deb`

## Adding a New OS

Adding a new OS means creating a new YAML file in `Build/platform/`. No Rust code changes are needed because the `PlatformRegistry` loads all YAML files at compile time via `include_dir!`.

Example: adding Alpine Linux

```yaml
# Build/platform/alpine.yaml
name: alpine
family: linux
manager: apk
install: apk add {{package}}
search: apk search {{package}}
provides: apk info -e {{package}}
sudo: true
repos:
  - alpine_3_20
```

## Detection

OS detection is handled by the `detect` function in `upi-core`. It reads `/etc/os-release` on Linux, `sw_vers` on macOS, and environment variables on Windows.

The detected OS is matched against the YAML definitions by name. Users can override detection with the `--os` flag.

## Next Steps

::: grids
::: grid
::: button "Resolution Pipeline" ./resolution.md icon:git-merge
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
::: grid
::: button "Architecture" ../reference/architecture.md icon:box
:::
:::
