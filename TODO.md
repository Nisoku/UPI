# TODO

- [ ] Add indicator while UPI is loading
- [ ] Language package managers? with an autodetect setup at the beginning that detects all managers you have installed
- [ ] Caching everywhere possible to ease the burden on Repology and servers and stuff

## Database Integration

- [ ] Add DB update mechanism (GitHub Actions (look at how chromebrew does it))

## Docs + CI

- [ ] Add Docs/ with architecture and YAML schema (docmd, duh)
- [ ] Add GitHub CI
- [ ] Add contribution guide + miscellaneous other things we need

## Install Script

- [ ] Write install.sh for cross platform installing without needing `cargo`
- [ ] Add GitHub Releases CI job for binary uploads

## Seed DB

- [ ] Add DB update mechanism (GitHub Actions, every 2-3 weeks)
- [X] Expand from 20 to 50+ common packages

## Rust Tooling

- [ ] Nightly Rust + Clippy pedantic CI gate
- [ ] fuzz testing for YAML parsing

## Future

- [ ] GUI for UPI (hehe)
- [ ] Maybe getting the core WASM compat for funsies hehe (web db and checker ooohhhhhhhh maybe that could be the web demo)
