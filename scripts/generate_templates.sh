#!/bin/bash
# Script to extract default Rust templates from OpenAPI Generator

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Extracting Rust templates from OpenAPI Generator..."

cd "$PROJECT_ROOT" || exit 1

openapi-generator-cli author template \
  -g rust \
  -o ./templates
