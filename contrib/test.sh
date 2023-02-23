#!/bin/sh

set -ex

FEATURES="std unicode-normalization rand serde zeroize"
LANGUAGES="chinese-simplified chinese-traditional czech french italian japanese korean spanish"

cargo --version
rustc --version

# Defaults / sanity checks
cargo build
cargo test

# Test each feature
if [ "$DO_FEATURE_MATRIX" = true ]; then
    cargo build --no-default-features
    cargo test --no-default-features

    # All features
    cargo build --no-default-features --features="$FEATURES"
    cargo test --no-default-features --features="$FEATURES"

    # Single features
    for feature in ${FEATURES}
    do
        cargo build --no-default-features --features="$feature"
        cargo test --no-default-features --features="$feature"
		# All combos of two features
		for featuretwo in ${FEATURES}; do
			cargo build --no-default-features --features="$feature $featuretwo"
			cargo test --no-default-features --features="$feature $featuretwo"
		done
    done
fi

# Test each language with std
for language in ${LANGUAGES}
do
    cargo test --verbose --features="$language"
done

# Bench if told to, must be run with non-stable (nightly, beta) toolchain.
if [ "$DO_BENCH" = true ]; then
    cargo bench
fi
