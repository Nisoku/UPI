# Contributing

Thank you for your interest in contributing to UPI.

## Code of Conduct

Be respectful, constructive, and inclusive.

## Reporting Bugs

Open an issue on [GitHub](https://github.com/Nisoku/UPI/issues) with:

- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your OS and UPI version (`upi --version`)

## Feature Requests

Open a discussion on [GitHub Discussions](https://github.com/Nisoku/UPI/discussions).

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just test` and `just lint`
5. Submit a PR

## Coding Guidelines

- Follow the existing code style
- No bare `.unwrap()` -- use `.expect()` with an invariant message
- No magic numbers -- name all constants
- Doc comments on all public API items
- Tests go in `Build/tests/`, not inline in source files
- OS behavior goes in YAML, not Rust
