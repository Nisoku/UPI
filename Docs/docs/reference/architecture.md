---
title: "Architecture"
description: "Codebase structure and design principles"
---

## Repository Layout

```txt
UPI/
  Cargo.toml                   # workspace root
  Build/
    crates/
      upi-core/                # resolver, DB, OS detection, YAML loader, exec
        data/                  # SQL schema, seed DB, version
        platform/              # YAML OS definitions (SSOT)
      upi-cli/                 # thin CLI wrapper (clap)
      upi-net/                 # network logic (ureq, isolated for WASM)
    tests/                     # all integration tests
    tools/
      db-update/               # seed DB auto-generation from Repology
  Docs/                        # documentation site (docmd)
  fuzz/                        # fuzz targets
  install.sh                   # curl | sh installer for Unix
  install.ps1                  # iwr | iex installer for Windows
```

## Crate Responsibilities

### upi-core

The heart of UPI. Zero network dependencies and zero CLI logic. Contains:

- **Resolver**: multi-source resolution pipeline (DB, Repology, fallback, identity)
- **Database**: SQLite-backed package mapping store with embedded seed DB
- **OS Detection**: platform detection via `/etc/os-release`, `sw_vers`, environment
- **YAML Loader**: compile-time embedding of OS definitions via `include_dir!`
- **Exec Layer**: command template expansion, sudo wrapping, execution

### upi-cli

A thin wrapper around upi-core. Contains:

- Argument parsing via `clap`
- Spinner and progress display via `indicatif`
- Dispatch to core resolver and exec

Zero resolver logic. Zero network logic.

### upi-net

Network logic isolated from the core for WASM compatibility. Contains:

- **RepologyClient**: HTTP client for the Repology API
- **RepologyResponse parsing**: repo, binname, srcname mapping
- **Dynamic repo-to-OS matching**: via `PlatformRegistry`

## Design Principles

### Data over Code

OS behavior lives in YAML, not Rust. Adding a new OS means creating a new YAML file. No code changes needed.

### Core over CLI

All logic lives in `upi-core`. The CLI is a thin dispatch layer. This enables future frontends (GUI, WASM, VS Code extension) to reuse the same core.

### Deterministic Resolution

Same input always produces the same output across machines. The resolution pipeline is deterministic and ordered by confidence.

### Offline First

UPI ships with a complete seed database. Common packages resolve without any network access.

### Transparent

Users always understand what UPI is doing. The `search` command shows every resolution attempt with its source and confidence. `--dry-run` shows the exact command before execution.

## WASM Compatibility

`upi-core` is designed for WASM compilation. It has:

- Zero network dependencies
- Zero filesystem dependencies at the core level
- Pure Rust dependencies throughout

The WASM build would expose the resolver for browser-based package name resolution without installation.

## Next Steps

::: grids
::: grid
::: button "Core Concepts" ../getting-started/concepts.md icon:book
:::
::: grid
::: button "Resolution Pipeline" ../guide/resolution.md icon:git-merge
:::
::: grid
::: button "Database Reference" ./database.md icon:database
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
