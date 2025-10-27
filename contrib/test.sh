#!/bin/sh

set -ex

FEATURES="serde rand all-languages chinese-simplified chinese-traditional czech french italian japanese korean portuguese spanish"
RUSTC_MINOR_VERSION=$(rustc --version | awk '{ split($2,a,"."); print a[2] }')

cargo --version
rustc --version

# Check the minimal lock file for all MSRV toolchains we test.
if [ "$RUSTC_MINOR_VERSION" -eq 56 ]; then
    cp Cargo-minimal.lock Cargo.lock
    cargo check --locked
    rm Cargo.lock
fi

# The crate's lowest MSRV (not lifted by `bitcoin_hashes` dep or `zeroize` feature). 
if [ "$RUSTC_MINOR_VERSION" -eq 41 ]; then
    cargo update --package "bitcoin_hashes" --precise "0.12.0"
    cargo update --package "rand" --precise "0.6.0"
    cargo update --package "libc" --precise "0.2.151"
    cargo update --package "tinyvec" --precise "1.6.0"
    cargo update --package "unicode-normalization" --precise "0.1.22"
fi

# `bitcoin_hashes` version raises the crate MSRV, we test with these toolchains explicitly in CI.
[ "$RUSTC_MINOR_VERSION" -eq 48 ]  && cargo update --package "bitcoin_hashes" --precise "0.13.0"
[ "$RUSTC_MINOR_VERSION" -eq 56 ]  && cargo update --package "bitcoin_hashes" --precise "0.14.0"

echo "********* Testing std *************"
# Test without any features other than std first
cargo test --verbose --no-default-features --features="std"

echo "********* Testing default *************"
# Then test with the default features
cargo test --verbose

# Build specific features
for feature in ${FEATURES}
do
    cargo build --verbose --features="$feature" --no-default-features
done

if [ "$RUSTC_MINOR_VERSION" -ge 51 ]; then
    cargo build --verbose --features="zeroize" --no-default-features
fi

if [ "$DO_NO_STD" = true ]
then
    echo "********* Testing no-std build *************"
    # Build no_std, to make sure that cfg(test) doesn't hide any issues
    cargo build --verbose --no-default-features

    # Build std + no_std, to make sure they are not incompatible
    cargo build --verbose
    # Test no_std
    cargo test --verbose --no-default-features

    # Build all features
    cargo build --verbose --features="$FEATURES" --no-default-features

    # Build specific features
    for feature in ${FEATURES}
    do
        cargo build --verbose --features="$feature" --no-default-features
    done

    if cargo --version | grep -v "1\.41"; then
        cargo build --verbose --features="zeroize" --no-default-features
    fi
fi

# Test each feature
for feature in ${FEATURES}
do
    echo "********* Testing $feature *************"
    cargo test --verbose --features="$feature"
done
