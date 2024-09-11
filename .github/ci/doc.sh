#!/bin/bash
## on push branch=main

set -euxo pipefail

export RUSTUP_HOME=/ci/cache/rustup
export CARGO_HOME=/ci/cache/cargo
export CARGO_TARGET_DIR=/ci/cache/target
export BUILDER_THREADS=4
export BUILDER_COMPRESS=true

# force rustup to download the toolchain before starting building.
# Otherwise, the docs builder is running multiple instances of cargo rustdoc concurrently.
# They all see the toolchain is not installed and try to install it in parallel
# which makes rustup very sad
rustc --version > /dev/null

docserver-builder -i . -o webroot/crates/rp-pac/git.zup

export KUBECONFIG=/ci/secrets/kubeconfig.yml
POD=$(kubectl -n embassy get po -l app=docserver -o jsonpath={.items[0].metadata.name})
kubectl cp webroot/crates $POD:/data
