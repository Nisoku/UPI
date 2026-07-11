---
title: "Resolution Pipeline"
description: "How UPI resolves a package name to an install command"
---

UPI resolves a generic package name to an OS-specific install command through a multi-source pipeline. Each source is checked in order, and the first match with sufficient confidence wins. Identity pass-through is **disabled by default** so use `--allow-identity` to enable it.

## Pipeline Order

### 1. Alias Resolution

Common shorthand is mapped to canonical names before any lookup:

| Alias | Resolves to |
|-------|-------------|
| `rg`  | `ripgrep`   |
| `py`  | `python`    |
| `nvim`| `neovim`    |
| `ff`  | `ffmpeg`    |
| `node`| `nodejs`    |
| `yt`  | `yt-dlp`    |

Aliases are checked first and short-circuit the entire pipeline.

### 2. Repology (confidence: 0.9)

UPI queries the [Repology](https://repology.org) API. Repology maintains a global index of how projects are packaged across 200+ repositories.

The client sends the query as a project name, receives all known package names across all repos, and matches them against the target OS's configured repositories using the `PlatformRegistry`. If Repology produces a match, resolution completes immediately.

### 3. Database Lookup (confidence: 1.0)

If Repology has no match, the embedded SQLite seed database is checked next. If the package exists in the database with a mapping for the target OS, resolution completes immediately.

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

### 4. Fallback Search (confidence: 0.7)

If neither Repology nor the database produce a result, UPI runs the native package manager's search command and applies heuristics to find the best match. The search command template is defined in the OS's YAML file.

**Short-query strict mode:** Queries of 3 characters or fewer skip fuzzy fallback entirely — only exact and alias matches are accepted. This prevents ambiguous short queries from resolving to unrelated packages.

### 5. Identity (opt-in)

As a last resort, the query is passed through as the package name. This succeeds when the generic name happens to match the native package name exactly.

Identity is **disabled by default**. When no confident match is found, UPI shows:

```text
error: resolve error: no confident match for 'foo'. Did you mean: foobar, foobar-git? Re-run with --allow-identity to install the exact input
```

Use `--allow-identity` to permit installing the raw query.

## Source Weighting

Each mapping in the database carries a confidence score:

| Source           | Confidence | Description                   |
|------------------|------------|-------------------------------|
| Database (exact) | 1.0        | Hardcoded, verified mapping   |
| Repology         | 0.9        | Live network lookup           |
| Fallback search  | 0.7        | Native package manager search |
| Identity         | 0.5        | Pass-through (opt-in)         |

Higher confidence scores indicate more reliable mappings, but the pipeline uses first-match-wins: the first source to produce a result determines the outcome.

## Offline Behavior

In `--offline` mode, the pipeline skips steps 2 and 4 entirely. Only the database and identity fallback (if enabled) are used. This guarantees zero network access.

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
