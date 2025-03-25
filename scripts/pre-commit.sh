#! /usr/bin/env bash

export RUSTFLAGS="-Dwarnings"
export RUSTDOCFLAGS="--cfg docsrs -Dwarnings"

set -e
set -x

cargo +nightly --version
cargo +nightly fmt --check
cargo +nightly clippy --all-features
cargo +nightly test
cargo +nightly test --all-features
cargo +nightly doc --no-deps -Zrustdoc-map --all-features
if command -v cargo-rdme 2>&1 >/dev/null; then
  cargo-rdme --check
else
  echo "Skip cargo-rdme (not installed)"
fi
