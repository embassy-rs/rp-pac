#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

(cd ../../chiptool/; cargo build)
RUST_LOG=info ../../chiptool/target/debug/chiptool generate --svd svd/rp2040.svd --transform svd/rp2040.yaml

form -i lib.rs -o src
rm lib.rs
#mv lib.rs src

cargo fmt
cargo check