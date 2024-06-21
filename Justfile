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
    grep -Fq '{{ version }}' CHANGELOG.md                 # The CHANGELOG.md should contains updated changes
    git diff --no-ext-diff --quiet --exit-code              # All files should be committed
    git tag -a v{{ version }} -m "Release v{{ version }}"
    git push origin v{{ version }}

# crate tag push it to origin and then publish to crates.io (TODO)
crates: tag-version
    cargo publish

# Create man page
_gen-man:
    cp support/manpage/places.md support/manpage/places_fix.md
    sed -i "s|<version>|{{ version }}|" support/manpage/places_fix.md
    sed -i "s|<date>|$(date '+%Y-%m-%d')|" support/manpage/places_fix.md
    pandoc --standalone --to man support/manpage/places_fix.md -o support/manpage/places.1
    rm support/manpage/places_fix.md

# Create and view the manpage
man: _gen-man
    PAGER=most man support/manpage/places.1

# Create a debian file
deb:
    cargo deb
    dpkg-deb --info target/debian/places-cli_{{ version }}-1_amd64.deb
    dpkg-deb --contents target/debian/places-cli_{{ version }}-1_amd64.deb
