#!/bin/bash
set -e

echo "=== Running Cargo Format Check ==="
cargo fmt --all -- --check

echo "=== Running Cargo Clippy (Denying Warnings) ==="
cargo clippy --workspace --all-targets -- -D warnings

echo "=== Running Cargo Tests ==="
cargo test --workspace

echo "=== Running Example Verification ==="
cargo check --examples

echo "Validation Gate: ALL PASS"
