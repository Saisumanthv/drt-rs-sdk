#!/bin/sh

# builds all wasm targets

cargo install dharitri-sc-meta --force

TARGET_DIR=target

sc-meta all build --target-dir-all $TARGET_DIR --path ./contracts
