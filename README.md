bip39
=====

A Rust implementation of [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
mnemonic codes.


## Word lists (languages)

We support all languages
[specified in the BIP-39 standard](https://github.com/bitcoin/bips/blob/master/bip-0039/bip-0039-wordlists.md)
as of writing.

The English language is always loaded and other languages can be loaded using the corresponding feature.

Use the `all-languages` feature to enable all languages.

- English (always enabled)
- Simplified Chinese (`chinese-simplified`)
- Traditional Chinese (`chinese-traditional`)
- Czech (`czech`)
- French (`french`)
- Italian (`italian`)
- Japanese (`japanese`)
- Korean (`korean`)
- Portuguese (`portuguese`)
- Spanish (`spanish`)


## Compact English word storage

The optional `compact-wordlist` feature stores English words in a single 11,068-byte string,
with a 1,024-byte index instead of a separate pointer/length pair for every word. Each group of
sixteen words has one 64-bit index entry containing a 16-bit starting offset and sixteen 3-bit
lengths, encoded as length minus three. Accessing a word sums at most fifteen preceding lengths
and returns a borrowed slice of the string without a UTF-8 conversion. English word lookup uses
binary search over these indexed words. Other languages keep their existing storage.

A small macro generates both the original word array and the compact string from one canonical
list of literals. The index is generated at compile time, with checks that offsets and lengths
fit the format.

Use `Language::word_at`, `Language::words_by_prefix_iter`, and `Mnemonic::words` to access words
without retaining the original English table. Mnemonic generation, parsing, lookup, and seed
derivation also use compact storage when this feature is enabled. No heap allocation is needed
to access a word, and word contents and BIP-39 indices are unchanged.

All existing APIs remain available. Calling `Language::word_list` or the deprecated
`Language::words_by_prefix` retains the original table of string references. Applications using
both representations can have a larger binary; size savings depend on the linker discarding
the unused original table. The feature is disabled by default.


## MSRV

This crate supports Rust v1.41.1 and up and works with `no_std`.

The `bitcoin_hashes` range dependency effects the MSRV as follows

- `bitcoin_hashes v0.12`: MSRV v1.41.1
- `bitcoin_hashes v0.13`: MSRV v1.48.0
- `bitcoin_hashes v0.14`: MSRV v1.56.0

When using older version of Rust, you might have to pin the versions of several crates, for an up-to-date list refer to [`contrib/test.sh`](contrib/test.sh):

```bash
cargo update --package "bitcoin_hashes" --precise "0.12.0"
cargo update --package "rand" --precise "0.7.0"
cargo update --package "libc" --precise "0.2.151"
cargo update --package "tinyvec" --precise "1.6.0"
cargo update --package "unicode-normalization" --precise "0.1.22"
cargo update --package "ppv-lite86" --precise "0.2.17"
```

If you enable the `zeroize` feature the MSRV becomes 1.51.

The `compact-wordlist` feature requires Rust 1.46 or newer for compile-time table generation.
