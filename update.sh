#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

cargo run --manifest-path ../svd2rust/Cargo.toml -- -i svd/rp2040.svd -c svd/rp2040.yaml

form -i lib.rs -o src
rm lib.rs

cargo fmt
cargo doc
rm -rf docs
mv target/doc docs