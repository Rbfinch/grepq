# Run tests and publish if successful
publish:
    #!/usr/bin/env sh
    VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f2)
    if ! grep -q "^$VERSION\$" CHANGELOG.md; then
        echo "Error: Version $VERSION not found in CHANGELOG.md"
        exit 1
    fi
    if bats test/test.bats && cargo test; then
        cargo publish
    else
        echo "Tests failed, skipping publish"
        exit 1
    fi
