#!/bin/sh

set -ex

FEATURES="serde rand all-languages"

cargo --version
rustc --version

# Pin dependencies as required if we are using MSRV toolchain.
# if cargo --version | grep "1\.41"; then
# fi

echo "********* Testing std *************"
# Test without any features other than std first
cargo test --verbose --no-default-features --features="std"

echo "********* Testing default *************"
# Then test with the default features
cargo test --verbose

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
fi

# Test each feature
for feature in ${FEATURES}
do
    echo "********* Testing $feature *************"
    cargo test --verbose --features="$feature"
done
