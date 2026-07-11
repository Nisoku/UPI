---
type: concept
title: "Quick Start"
description: "Install your first package with UPI in 30 seconds"
source: "https://nisoku.org/UPI/getting-started/quickstart/"
path: /getting-started/quickstart/
updated: 2026-07-11
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-11T02:38:05.511Z"
---
---
title: "Quick Start"
description: "Install your first package with UPI in 30 seconds"
---

This guide gets you from zero to running your first cross-platform install with UPI.

::: callout tip "Prerequisites"
You need Rust 1.75+. See [Installation](./installation) if you do not have it yet.
:::

## Install UPI

```bash
cargo install upi
```

## Install a Package

```bash
upi install vim
```

UPI detects your OS, resolves the correct package name, and runs the install command.

## See What Would Run

Use `--dry-run` to see the command without executing it:

```bash
upi --dry-run install vim
# macOS:   brew install vim
```

## Try Different OSes

UPI can generate install commands for any supported platform, regardless of your current OS:

```bash
upi --os debian --dry-run install vim     # sudo apt install -y vim
upi --os fedora --dry-run install vim     # sudo dnf install -y vim
upi --os arch --dry-run install vim       # sudo pacman -S --noconfirm vim
upi --os windows --dry-run install vim    # sudo choco install -y vim
```

## Search for a Package

```bash
upi search vim
```

Shows all known names for a package across your OS, with their resolution source and confidence level.

## Offline Mode

```bash
upi --offline --dry-run install vim
```

Resolves entirely from the built-in database. No network calls at all.

## Next Steps

::: grids
::: grid
::: button "Installation" ./installation.md icon:download
:::
::: grid
::: button "Core Concepts" ./concepts.md icon:book
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
::: grid
::: button "Architecture" ../reference/architecture.md icon:box
:::
:::
