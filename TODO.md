# TODO

## Database Integration

- [ ] Add DB update mechanism (GitHub Actions (look at how chromebrew does it))

## Logging + UX Polish

- [X] Add verbosity flags (clap)
- [X] Add pretty output
- [X] Add error formatting
- [X] Add timing and debug logs (`log` + `env_logger`)
- [X] `--os` override flag (test install commands for any platform)
- [X] `search` subcommand (`upi search <query>`)

## Docs + CI

- [ ] Add Docs/ with architecture and YAML schema (docmd, duh)
- [ ] Add GitHub CI
- [ ] Add contribution guide + miscellaneous other things we need

## Install Script

- [ ] Write install.sh for cross platform installing without needing `cargo`
- [ ] Add GitHub Releases CI job for binary uploads

## Seed DB

- [ ] Add DB update mechanism (GitHub Actions)
- [ ] Expand from 20 to 50+ common packages

## Rust Tooling

- [ ] Nightly Rust + Clippy pedantic CI gate
- [ ] fuzz testing for YAML parsing

## Future

- [ ] GUI for UPI (hehe)
- [ ] Maybe getting the core WASM compat for funsies hehe (web db and checker ooohhhhhhhh maybe that could be the web demo)
