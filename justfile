set export

RUSTFLAGS := "-Dwarnings"
RUSTDOCFLAGS := "--cfg docs -Dwarnings"

check *FLAGS:
    cargo fmt --check
    cargo test
    cargo test --all-targets --all-features
    cargo doc -Zrustdoc-map --all-features

doc *FLAGS:
    cargo +nightly doc -Zrustdoc-map --all-features {{FLAGS}}

readme *FLAGS:
    cargo rdme {{FLAGS}}
