#!/usr/bin/env bash

if ! command -v chiptool &> /dev/null; then
    echo "chiptool could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    echo ""
    exit 1
fi

if ! command -v form &> /dev/null; then
    echo "form could not be found. Install it with the following command:"
    echo ""
    echo "    cargo install form"
    echo ""
    exit 1
fi

set -euxo pipefail

rm -rf src/rp*

for chip in rp2040 rp235x; do 
    chiptool generate --svd svd/$chip.svd --transform svd/$chip.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs
    form -i lib.rs -o src/$chip
    mv src/$chip/lib.rs src/$chip/mod.rs
    rm lib.rs
done

cargo fmt
cargo check --features rp2040
cargo check --features rp235x
