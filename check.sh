#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

toolchain=$(sed -n 's/^channel = "\(.*\)"$/\1/p' rust-toolchain.toml)
if [[ -z "$toolchain" ]] || ! command -v rustup >/dev/null; then
    echo "check.sh requires rustup and a channel in rust-toolchain.toml" >&2
    exit 1
fi

cargo +"$toolchain" fmt --all -- --check
cargo +"$toolchain" test --locked
cargo +"$toolchain" clippy --locked --all-targets --all-features -- -D warnings
