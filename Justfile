build:
    cargo build

run:
    cargo run -p upi

test:
    cargo test

check:
    cargo check

clippy:
    cargo clippy --all-targets

clippy-fix:
    cargo clippy --fix --allow-dirty --all-targets

clean:
    cargo clean

release:
    cargo build --release

format: 
    cargo fmt

manifest:
    cargo publish --dry-run --allow-dirty -p upi-core

all: check format clippy test manifest
