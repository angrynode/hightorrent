#! /usr/bin/env bash

export RUSTFLAGS="-Dwarnings"
export RUSTDOCFLAGS="--cfg docsrs -Dwarnings"

set -e

cargo --version
rustc --version
cargo fmt --check
cargo clippy --all-features
cargo test
cargo test --all-features
cargo doc --no-deps -Zrustdoc-map --all-features
