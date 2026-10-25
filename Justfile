set positional-arguments

default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Build the libraries with the nightly pinned in rust-toolchain.toml.
build package="":
    #!/usr/bin/env bash
    set -euo pipefail
    selection=(--workspace)
    if [[ -n "$1" ]]; then selection=(-p "$1"); fi
    rtk cargo build "${selection[@]}"

# Clippy over the lint sources, then the libraries linting this repository.
lint package="":
    #!/usr/bin/env bash
    set -euo pipefail
    selection=(--workspace)
    if [[ -n "$1" ]]; then selection=(-p "$1"); fi
    rtk cargo clippy "${selection[@]}" --all-targets
    if [[ -z "$1" ]]; then
        just lint-self
    fi

# Dogfooding: rust_layout and rust_testing lint the crates of this repository.
lint-self:
    cargo dylint --all --workspace -- --all-targets

# nextest over unit tests, then the fixtures.
test package="":
    #!/usr/bin/env bash
    set -euo pipefail
    selection=(--workspace)
    if [[ -n "$1" ]]; then selection=(-p "$1"); fi
    rtk cargo nextest run "${selection[@]}" --no-tests warn
    if [[ -z "$1" ]]; then
        just test-fixtures
    fi

# Lint the good/bad fixtures with each library and compare with fixtures/expected.
test-fixtures:
    bash fixtures/run.sh
