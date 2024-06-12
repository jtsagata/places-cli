# Guard for running just without args. just list recipes
recipes-list:
    just --list

# Check code
check:
    cargo clippy --all-targets --all-features --workspace -- -D warnings
    cargo fmt --all -- --check

# Run clippy and format code
lint:
    cargo clippy --all
    cargo fmt --all --check

# Publish to gitea
gitea:
    git push gitea

# Load version from Cargo.toml
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`

# create version tag and push to origin
tag-version:
    cargo deny check
    grep -Fq '[{{ version }}]' CHANGELOG.md                  # The CHANGELOG.md should contains updated changes
    git diff --no-ext-diff --quiet --exit-code              # All files should be committed
    git tag -a {{ version }} -m "Release {{ version }}"
    git push origin {{ version }}

# crate tag push it to origin and then publish to crates.io
tag-and-publish: tag-version
    cargo publish
