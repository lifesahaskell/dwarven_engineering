#!/usr/bin/env bash
# Local mirror of .github/workflows/ci.yml — same checks, same order, fail-fast.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
