#!/usr/bin/env bash
set -euo pipefail

cargo +stable fmt --all -- --check
cargo +stable test --locked
cargo +stable clippy --locked --all-targets --all-features -- -D warnings
