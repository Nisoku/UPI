# Security

## Reporting a Vulnerability

If you discover a security vulnerability in UPI, please report it privately by opening a security advisory on [GitHub](https://github.com/Nisoku/UPI/security/advisories).

Please do not report security vulnerabilities through public GitHub issues.

## Dependency Audits

Dependencies are audited regularly via `cargo audit` in CI. The dependency tree is checked for known CVEs on every change to `Cargo.lock` and on a daily schedule.
