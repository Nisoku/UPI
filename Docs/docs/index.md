---
title: "UPI"
description: "Universal Package Installer - one command to install anything, anywhere"
---

::: hero layout:split glow:true

<!-- markdownlint-disable MD025 -->
# UPI

Universal Package Installer. One command to install anything, anywhere.

::: tag "5 OS Families"
::: tag "8 Package Managers"
::: tag "Zero Setup"
::: tag "Offline First"

::: button "Quick Start" ./getting-started/quickstart.md icon:play
::: button "GitHub" external:<https://github.com/Nisoku/UPI> icon:github

== side

::: card "What is UPI?"
UPI translates a generic package name into the correct native install command for your system. It is a translator and resolver over existing package managers, not a package manager itself.

```bash
upi install vim               # macOS:   brew install vim
upi --os debian install vim   # Debian:  apt install vim
upi --os arch install vim     # Arch:    pacman -S vim
upi --os windows install vim  # Windows: choco install vim
```

Works offline. Works everywhere.
:::

:::

## Features

::: grids
::: grid
::: card "Cross Platform" icon:globe
One command for macOS, Debian, Fedora, Arch, and Windows. UPI knows the right package name and the right package manager for every OS.
:::
:::

::: grid
::: card "Offline First" icon:database
Built-in compressed SQLite seed database with 10,089 mappings across 531 packages. Works immediately after install with no network required.
:::
:::

::: grid
::: card "Repology Powered" icon:search
When the local database does not have a match, UPI queries Repology's global package index for live resolution across 10,000+ projects.
:::
:::

::: grid
::: card "Multi Source Pipeline" icon:git-merge
Resolution follows a priority chain: database (confidence=1), then Repology, then package manager fallback, then identity. Each source is weighted and transparent.
:::
:::

::: grid
::: card "YAML Defined" icon:file-text
OS behavior lives in YAML files under `Build/platform/`. Adding a new OS means dropping a YAML file with no Rust changes required.
:::
:::

::: grid
::: card "Pure Rust" icon:box
Built entirely in Rust. No runtime dependencies, no Node.js, no Python. Single binary with embedded assets.
:::
:::

:::

## Quick Example

::: tabs

== tab "Search"

```bash
upi search vim
```

```text
 OS:       Macos
 Manager:  homebrew
 Query:    vim
 Results:
   vim                          <- database (confidence=1)
   neovim                       <- repology
   macvim                       <- repology
 Command:  brew install vim
```

== tab "Install (dry-run)"

```bash
upi --dry-run install python
```

Shows the command that would run without executing it. Works across any OS:

```bash
upi --os debian --dry-run install python   # -> sudo apt install -y python2.7
upi --os arch --dry-run install python     # -> sudo pacman -S --noconfirm python2
upi --os windows --dry-run install python  # -> sudo choco install -y python3
```

== tab "Offline"

```bash
upi --offline --dry-run install vim
```

No network calls. Resolves entirely from the built-in seed database.

:::

## Installation

```bash
cargo install upi
```

Or download a pre-built binary from the [releases page](https://github.com/Nisoku/UPI/releases).

## Next Steps

::: grids
::: grid
::: button "Quick Start" ./getting-started/quickstart.md icon:play
:::
::: grid
::: button "Installation" ./getting-started/installation.md icon:download
:::
::: grid
::: button "Core Concepts" ./getting-started/concepts.md icon:book
:::
::: grid
::: button "CLI Reference" ./cli/ icon:terminal
:::
::: grid
::: button "Architecture" ./reference/architecture.md icon:box
:::
::: grid
::: button "Resolution Pipeline" ./guide/resolution.md icon:git-merge
:::
:::
