help:
    @just --list

local-ci:
    cargo build
    cargo fmt --check
    cargo clippy --no-deps --all-targets
    cargo test
    cargo publish --dry-run --allow-dirty
