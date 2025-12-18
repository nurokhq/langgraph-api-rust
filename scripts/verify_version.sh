#!/bin/bash

set -e

# Extract version from tag
# Can be provided as argument or extracted from GITHUB_REF
if [ -n "$1" ]; then
    TAG_VERSION="$1"
    # Remove 'v' prefix if present
    TAG_VERSION="${TAG_VERSION#v}"
elif [ -n "$GITHUB_REF" ]; then
    TAG_VERSION="${GITHUB_REF#refs/tags/v}"
else
    echo "Error: No version provided. Usage: $0 <version> or set GITHUB_REF"
    echo "Example: $0 0.1.0 or $0 v0.1.0"
    exit 1
fi

echo "Verifying version: $TAG_VERSION"

# Extract version from Cargo.toml
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d '"' -f 2)
if [ -z "$CARGO_VERSION" ]; then
    echo "Error: Could not find version in Cargo.toml"
    exit 1
fi

echo "Cargo.toml version: $CARGO_VERSION"

# Extract versions from README.md
# Look for version in dependency declarations: version = "X.Y.Z"
README_VERSIONS=$(grep -o 'version = "[0-9]\+\.[0-9]\+\.[0-9]\+"' README.md | cut -d '"' -f 2 | sort -u)
if [ -z "$README_VERSIONS" ]; then
    echo "Warning: Could not find version declarations in README.md"
else
    echo "README.md versions found:"
    echo "$README_VERSIONS" | while read -r ver; do
        echo "  - $ver"
    done
fi

# Verify Cargo.toml matches tag
if [ "$CARGO_VERSION" != "$TAG_VERSION" ]; then
    echo "Error: Cargo.toml version ($CARGO_VERSION) does not match tag version ($TAG_VERSION)"
    exit 1
fi

# Verify README versions match tag
if [ -n "$README_VERSIONS" ]; then
    while IFS= read -r readme_ver; do
        if [ "$readme_ver" != "$TAG_VERSION" ]; then
            echo "Error: README.md version ($readme_ver) does not match tag version ($TAG_VERSION)"
            exit 1
        fi
    done <<< "$README_VERSIONS"
fi

echo "✓ All versions match: $TAG_VERSION"
exit 0