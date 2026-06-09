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

fmt:
    cargo fmt

clean:
    cargo clean

release:
    cargo build --release

all: check test
