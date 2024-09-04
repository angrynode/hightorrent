set export

RUSTFLAGS := "-Dwarnings"
RUSTDOCFLAGS := "--cfg docs -Dwarnings"

check *FLAGS:
    ./scripts/pre-commit.sh {{FLAGS}}

doc *FLAGS:
    cargo +nightly doc -Zrustdoc-map --all-features {{FLAGS}}

readme *FLAGS:
    cargo rdme {{FLAGS}}
