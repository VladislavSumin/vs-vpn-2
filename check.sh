#!/usr/bin/env bash
set -euo pipefail

echo "=== cargo build ==="
cargo build

echo "=== cargo test ==="
cargo test

echo "=== cargo fmt --check ==="
cargo fmt --all -- --check

echo "=== cargo clippy ==="
cargo clippy -- -D warnings

echo "=== All checks passed ==="
