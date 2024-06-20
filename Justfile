# Load version from Cargo.toml
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`


# Guard for running just without args. just list recipes
_default:
    just --list

# Run clippy and format code
check: _shellcheck _format
    cargo clippy --all

# Format code
_format:
    cargo fmt --all --check
    cargo fmt --all

# Check shell scripts
_shellcheck:
    fd -e sh -t f -X shellcheck

# create version tag and push to origin
tag-version: check
    cargo deny check
    grep -Fq '[{{ version }}]' CHANGELOG.md                 # The CHANGELOG.md should contains updated changes
    git diff --no-ext-diff --quiet --exit-code              # All files should be committed
    git tag -a {{ version }} -m "Release {{ version }}"
    git push origin {{ version }}

# crate tag push it to origin and then publish to crates.io (TODO)
crates: tag-version
    cargo publish

# Create man page
_gen-man:
    cp support/manpage/places.md support/manpage/places.md.bk
    sed -i.bk "s|footer: places <version>|footer: places {{ version }}|" support/manpage/places.md
    sed -i.bk "s|date: <date>|date: $(date '+%Y-%m-%d')|" support/manpage/places.md
    pandoc --standalone --to man support/manpage/places.md -o support/manpage/places.1
    cp support/manpage/places.md.bk support/manpage/places.md
    rm support/manpage/places.md.bk

# Create and view the manpage
man: _gen-man
    man support/manpage/places.1
