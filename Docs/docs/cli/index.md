---
title: "CLI Reference"
description: "Full reference for the UPI command-line interface"
---

## Usage

```txt
upi [OPTIONS] <COMMAND>
```

## Global Options

| Option              | Description                                               |
|---------------------|-----------------------------------------------------------|
| `--dry-run`         | Show the install command without executing it             |
| `--offline`         | Skip network lookups; use only the local database         |
| `--os <OS>`         | Override the target OS, like `macos`, `debian`, or `arch` |
| `-v`, `-vv`, `-vvv` | Increase verbosity (info, debug, trace)                   |
| `-h`, `--help`      | Print help information                                    |
| `-V`, `--version`   | Print version information                                 |

## Commands

### install

Install a package by name.

```bash
upi install <package>
```

UPI detects the current OS, resolves the package name, and executes the install command. Use `--dry-run` to preview the command without running it.

**Examples:**

```bash
upi install vim
upi --dry-run install python
upi --os debian --dry-run install ffmpeg
upi --offline --dry-run install neovim
```

### search

Search for package name mappings for a given query.

```bash
upi search <package>
```

Shows all known package names from all resolution sources, with their source and confidence level.

**Example output:**

```txt
 OS:       Macos
 Manager:  homebrew
 Query:    python
 Results:
   python@3.12       <- database (confidence=1)
   mod_python27      <- repology
   boost-python3     <- fallback search
   python            <- identity
 Command:  brew install python@3.12
```

## Verbosity Levels

| Level          | Flag   | Output                                |
|----------------|--------|---------------------------------------|
| Warn (default) |        | Errors and warnings only              |
| Info           | `-v`   | High-level progress messages          |
| Debug          | `-vv`  | Detailed debug output                 |
| Trace          | `-vvv` | Full tracing, including HTTP requests |

## Exit Codes

| Code | Meaning                       |
|------|-------------------------------|
| 0    | Success                       |
| 1    | Resolution or execution error |

## Environment Variables

| Variable   | Description                                                |
|------------|------------------------------------------------------------|
| `CI`       | When set, hides the spinner animation (useful for CI/CD)   |
| `RUST_LOG` | Overrides the log level (takes precedence over `-v` flags) |

## Next Steps

::: grids
::: grid
::: button "Quick Start" ../getting-started/quickstart.md icon:play
:::
::: grid
::: button "Resolution Pipeline" ../guide/resolution.md icon:git-merge
:::
::: grid
::: button "Architecture" ../reference/architecture.md icon:box
:::
:::
