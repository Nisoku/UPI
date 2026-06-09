# TODO

## Core Skeleton

- [x] Add error module
- [x] Add os module with detection + loader stubs
- [x] Add resolver module stub
- [x] Add exec module stub
- [x] Add db module stub
- [x] Add repology stub

## YAML OS Definitions

- [ ] Add platform YAML schema (noyalib)
- [ ] Add macOS YAML (Homebrew)
- [ ] Add Debian/Ubuntu YAML (apt)
- [ ] Add Fedora YAML (dnf)
- [ ] Add Arch YAML (pacman)
- [ ] Add Windows YAML (winget)
- [ ] Add YAML loader that reads into a registry

## Resolver Pipeline

- [ ] Implement platform detection
- [ ] Load YAML registry
- [ ] Resolve package name (identity mapping stub)
- [ ] Build install command from YAML template
- [ ] Execute or dry-run

## Database Integration

- [ ] Add SQLite schema (rusqlite)
- [ ] Add seed DB loader (include_bytes! + ruzstd)
- [ ] Add DB lookup in resolver
- [ ] Add DB update mechanism (GitHub Releases)

## Repology Integration (upi-net)

- [ ] Implement Repology client (ureq + serde_json)
- [ ] Stream deserialize project to package mapping
- [ ] Integrate into resolver pipeline

## Fallback Search using system package manager

- [ ] Add fallback search commands from YAML
- [ ] Integrate into resolver pipeline
- [ ] Add heuristics for best match

## UX Polish

- [ ] Add verbosity flags (clap)
- [ ] Add pretty output
- [ ] Add error formatting
- [ ] Add timing and debug logs (log + env_logger)

## Docs + CI

- [ ] Add Docs/ with architecture and YAML schema (docmd, duh)
- [ ] Add GitHub CI
- [ ] Add contribution guide + miscellaneous other things we need

## Install Script

- [ ] Write install.sh for cross platform installing without needing `cargo`
- [ ] Add GitHub Releases CI job for binary uploads

## Future

- [ ] GUI for UPI (hehe)
- [ ] Maybe getting the core WASM compat for funsies hehe (web db and checker ooohhhhhhhh maybe that could be the web demo)
