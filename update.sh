#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

#(cd ../chiptool/; cargo build)
#RUST_LOG=info ../chiptool/target/debug/chiptool generate --svd svd/rp2040.svd --transform svd/rp2040.yaml
chiptool generate --svd svd/rp2040.svd --transform svd/rp2040.yaml

# cargo install form
form -i lib.rs -o src
rm lib.rs

cargo fmt
sed -i '/#!\[doc =/c\#![doc = include_str!("../README.md")]\n#![allow(non_camel_case_types)]' src/lib.rs

cargo check
