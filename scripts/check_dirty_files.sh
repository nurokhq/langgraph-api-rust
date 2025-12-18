#!/bin/bash

set -e

./scripts/generate_api_sdk.sh

if git status --porcelain | grep -q "M"; then
    echo "Error: There are dirty files"
    exit 1
fi