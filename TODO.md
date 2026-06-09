# TODO

## Database Integration

- [ ] Add DB update mechanism (GitHub Releases)

## Repology Integration (upi-net)

- [X] Implement Repology client (ureq + serde_json)
- [X] Stream deserialize project to package mapping
- [X] Integrate into resolver pipeline
- [X] Add tests for Repology

## Fallback Search using system package manager

- [ ] Add fallback search commands from YAML
- [ ] Integrate into resolver pipeline
- [ ] Add heuristics for best match
- [ ] Add tests for fallback

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
