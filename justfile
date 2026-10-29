default:
    @just --list

build:
    cargo build --locked

test:
    cargo test --locked --all-features

lint:
    cargo clippy --locked --all-targets --all-features -- -D warnings

format:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

check: fmt-check lint test
