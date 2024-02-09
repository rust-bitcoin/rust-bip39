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


## MSRV

This crate supports Rust v1.41.1 and up and works with `no_std`.

When using older version of Rust, you might have to pin the version of the
`bitcoin_hashes` crate used as such:
```
$ cargo update --package "bitcoin_hashes" --precise "0.12.0"
```

If you enable the `zeroize` feature the MSRV becomes 1.51.
