help:
    @just --list

local-ci:
    cargo build
    cargo fmt --check
    cargo clippy --no-deps --all-targets
    cargo test
    cargo publish --dry-run --allow-dirty

docker-build *args:
    docker build -t miasma:local-dev . {{args}}

docker-run *args:
    docker run --rm -p 9999:9999 miasma:local-dev {{args}}
