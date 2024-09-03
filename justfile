set export

RUSTFLAGS := "-Dwarnings"
RUSTDOCFLAGS := "--cfg docs -Dwarnings"

doc *FLAGS:
    cargo +nightly doc -Zrustdoc-map --all-features {{FLAGS}}
