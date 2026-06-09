# TODO

## Core thingy

- [ ] Add error module
- [ ] Add os module with detection + loader stubs
- [ ] Add resolver module stub
- [ ] Add exec module stub
- [ ] Add db module stub
- [ ] Add repology stub

## YAML OS Definitions

- [ ] Add platform YAML schema
- [ ] Add macOS YAML (Homebrew)
- [ ] Add Debian/Ubuntu YAML (apt)
- [ ] Add Fedora YAML (dnf)
- [ ] Add Arch YAML (pacman)
- [ ] Add Windows YAML (winget)
- [ ] Add YAML loader that reads into a registry

## Resolver

- [ ] Implement platform detection
- [ ] Load YAML registry
- [ ] Resolve package name (identity mapping stub)
- [ ] Build install command from YAML template
- [ ] Execute or dry-run

## DB Integration

- [ ] Add SQLite schema
- [ ] Add seed DB loader (include_bytes)
- [ ] Add DB lookup in resolver
- [ ] Add DB update mechanism (GitHub Releases)

## Repology

- [ ] Add Repology client
- [ ] Add project to package mapping
- [ ] Integrate into resolver pipeline

## Fallback Search using system package manager

- [ ] Add fallback search commands from YAML
- [ ] Integrate into resolver pipeline
- [ ] Add heuristics for best match

## UX Polish

- [ ] Add verbosity flags
- [ ] Add pretty output
- [ ] Add error formatting
- [ ] Add timing and debug logs

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
