CHANGELOG
=========

# v2.0.0

- Set Rust edition to 2018
- Make `rand` and `rand_core` dependencies more flexible
  - Increase maximum version in constraint
  - Make `rand_core` optional too
  - Expose both crates
- Bump MSRV to 1.41.1

# v1.2.0

- Add `Mnemonic::parse_in_normalized_without_checksum_check`
- Make public `Mnemonic::normalize_utf8_cow`

# v1.1.0

- Add zerioze support through a feature (MSRV 1.51)
- Allow word count multiples of 3 instead of 6
- Add support for no-std usage
- Expose `Language::word_list` and `Language::find_word` methods

# v1.0.1

- Add `Mnemonic::language` getter.
- Make `Mnemonic::language_of` static method public.
- Change internal representation of `Mnemonic`, making it slightly smaller.

# v1.0.0

- Initial version of entire rewrite.
