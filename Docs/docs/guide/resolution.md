---
title: "Resolution Pipeline"
description: "How UPI resolves a package name to an install command"
---

UPI resolves a generic package name to an OS-specific install command through a multi-source pipeline. Each source is checked in order, and the first match with sufficient confidence wins.

## Pipeline Order

### 1. Database Lookup (confidence: 1.0)

The embedded SQLite seed database is checked first. If the package exists in the database with a mapping for the target OS, resolution completes immediately without any network calls.

```txt
query: "vim" on macos
  -> SELECT os_package FROM mappings
     WHERE package_id = "vim" AND os = "macos"
  -> "vim"
  -> brew install vim
```

Database mappings include:

- **repology_auto**: sourced from Repology scans during seed DB generation
- **winget_direct**: authoritative winget PackageId for Windows packages
- **derived**: inferred from package name patterns across OSes

### 2. Repology (confidence: 0.9)

When the database has no match, UPI queries the [Repology](https://repology.org) API. Repology maintains a global index of how projects are packaged across 200+ repositories.

The client sends the query as a project name, receives all known package names across all repos, and matches them against the target OS's configured repositories using the `PlatformRegistry`.

### 3. Fallback Search (confidence: 0.7)

If neither the database nor Repology produce a result, UPI runs the native package manager's search command and applies heuristics to find the best match. The search command template is defined in the OS's YAML file.

### 4. Identity (confidence: 0.5)

As a last resort, the query is passed through as the package name. This succeeds when the generic name happens to match the native package name exactly.

## Source Weighting

Each mapping in the database carries a confidence score:

| Source           | Confidence | Description                   |
|------------------|------------|-------------------------------|
| Database (exact) | 1.0        | Hardcoded, verified mapping   |
| Repology         | 0.9        | Live network lookup           |
| Fallback search  | 0.7        | Native package manager search |
| Identity         | 0.5        | Pass-through                  |

Higher confidence sources take priority when multiple sources produce results.

## Offline Behavior

In `--offline` mode, the pipeline skips steps 2 and 3 entirely. Only the database and identity fallback are used. This guarantees zero network access.

## Example Resolution

```bash
upi search python
```

```text
 OS:       Macos
 Manager:  homebrew
 Query:    python
 Results:
   python@3.12       <- database (confidence=1)
   boost-python3     <- fallback search
   python            <- identity
 Command:  brew install python@3.12
```

The database match at confidence=1 wins over the fallback and identity results.

## Next Steps

::: grids
::: grid
::: button "Supported Platforms" ../guide/platforms.md icon:globe
:::
::: grid
::: button "Database Reference" ../reference/database.md icon:database
:::
::: grid
::: button "Architecture" ../reference/architecture.md icon:box
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
