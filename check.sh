#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

toolchain=$(sed -n 's/^channel = "\(.*\)"$/\1/p' rust-toolchain.toml)
if [[ -z "$toolchain" ]] || ! command -v rustup >/dev/null; then
    echo "check.sh requires rustup and a channel in rust-toolchain.toml" >&2
    exit 1
fi

if [[ $(cargo --version) != "cargo $toolchain "* ]] ||
   [[ $(rustc --version) != "rustc $toolchain "* ]]; then
    echo "check.sh requires Cargo and rustc from Rust $toolchain" >&2
    exit 1
fi

cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
