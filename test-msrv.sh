#!/bin/sh

set -e

MSRV="1.41.1"

CMD="${RUSTUP:-rustup} run ${MSRV}"

rm -f Cargo.lock
$CMD cargo generate-lockfile
$CMD cargo update --package "bitcoin_hashes" --precise "0.12.0"
$CMD cargo update --package "rand" --precise "0.6.5"

$CMD cargo build --no-default-features
$CMD cargo test  --no-default-features
$CMD cargo build --no-default-features --features all-languages
$CMD cargo test  --no-default-features --features all-languages
$CMD cargo build --features rand_core,all-languages
$CMD cargo test  --features rand_core,all-languages
$CMD cargo build --features rand,all-languages
$CMD cargo test  --features rand,all-languages
