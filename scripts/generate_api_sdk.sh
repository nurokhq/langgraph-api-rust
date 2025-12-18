#!/bin/bash

set -e

openapi-generator-cli generate \
  -i ./api/langgraph-openapi.json \
  -g rust \
  -o ./api/langgraph-api \
  -t ./templates

rm -rf ./src/generated/apis
rm -rf ./src/generated/models
cp -r ./api/langgraph-api/src/apis ./src/generated/apis
cp -r ./api/langgraph-api/src/models ./src/generated/models
rm -rf ./api/langgraph-api

cargo fmt
