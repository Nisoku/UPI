---
title: "Core Concepts"
description: "How UPI works under the hood"
---

## What UPI Is

UPI is a **cross-OS macro installer**. It gives you a single, stable command:

```bash
upi install <package>
```

and translates it into the correct native install command for your system.

UPI is **not** a package manager. It is a **translator**, a **resolver**, and a **unified interface** over existing package managers.

## The Problem

Installing the same package on different OSes requires:

- Different **commands** (`brew install` vs `apt install` vs `pacman -S`)
- Different **package names** (`python` vs `python3` vs `python2`)
- Different **package managers** (Homebrew vs apt vs dnf vs pacman)

UPI removes this friction.

## Resolution Pipeline

UPI resolves package names through a priority-ordered pipeline:

::: grids
::: grid
::: card "1. Database Lookup" icon:database confidence:1
The compressed SQLite seed DB contains 10,089 mappings across 531 packages. When a match is found at confidence=1, resolution stops immediately with no network needed.
:::
:::

::: grid
::: card "2. Repology" icon:search confidence:0.9
If the database has no match, UPI queries the Repology API for project-to-package-name resolution. Repology indexes 10,000+ projects across 200+ repos.
:::
:::

::: grid
::: card "3. Fallback Search" icon:terminal confidence:0.7
If Repology does not return a result, UPI runs the native package manager's search command and applies heuristics to find the best match.
:::
:::

::: grid
::: card "4. Identity" icon:arrow-right confidence:0.5
As a last resort, the name is passed through as-is. This works when the generic name matches the native package name.
:::
:::

:::

## Offline First

UPI ships with a compressed SQLite seed database embedded in the binary. On first run it is decompressed to `~/.upi/db/seed.db` and cached with version checking. The database covers 531 common cross-platform packages with 10,089 package-manager-specific mappings.

Result: UPI works offline immediately for the vast majority of common packages.

## OS Definitions

All OS behavior is defined in YAML files under `Build/platform/`. Each file specifies:

- Install command template
- Search command template
- Package manager name
- Platform detection rules

Adding a new OS means creating a new YAML file with no Rust code changes required.

## Database

The seed database uses SQLite with these tables:

| Table      | Purpose                                            |
|------------|----------------------------------------------------|
| `meta`     | Schema version and build info                      |
| `packages` | Canonical package entries                          |
| `mappings` | OS-specific package names with provenance tracking |
| `aliases`  | Alternative names, like `python3` to `python`      |

Each mapping has a `source` field (`repology_auto`, `winget_direct`, `derived`) and a `confidence` score (0.0 to 1.0).

## Next Steps

::: grids
::: grid
::: button "Quick Start" ./quickstart.md icon:play
:::
::: grid
::: button "Resolution Pipeline" ../guide/resolution.md icon:git-merge
:::
::: grid
::: button "Architecture" ../reference/architecture.md icon:box
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
