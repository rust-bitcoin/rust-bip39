#![feature(test)]

extern crate bip39;
extern crate test;

use test::Bencher;

use bip39::*;

#[cfg(not(any(
	feature = "chinese-simplified", feature = "chinese-traditional", feature = "czech",
	feature = "french", feature = "italian", feature = "japanese", feature = "korean",
	feature = "spanish"
)))]
const LANG: Language = Language::English;
#[cfg(feature = "chinese-simplified")]
const LANG: Language = Language::SimplifiedChinese;
#[cfg(feature = "chinese-traditional")]
const LANG: Language = Language::TraditionalChinese;
#[cfg(feature = "czech")]
const LANG: Language = Language::Czech;
#[cfg(feature = "french")]
const LANG: Language = Language::French;
#[cfg(feature = "italian")]
const LANG: Language = Language::Italian;
#[cfg(feature = "japanese")]
const LANG: Language = Language::Japanese;
#[cfg(feature = "korean")]
const LANG: Language = Language::Korean;
#[cfg(feature = "spanish")]
const LANG: Language = Language::Spanish;

#[bench]
fn validate(b: &mut Bencher) {
    let entropy = b"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f";
    let mnemonic = Mnemonic::from_entropy_in(LANG, &entropy[..]).unwrap();
	assert_eq!(mnemonic.word_count(), 24);
	let phrase = mnemonic.as_str();

    b.iter(|| {
        let _ = Mnemonic::validate_in(Language::English, &phrase);
    });
}

#[bench]
fn from_entropy(b: &mut Bencher) {
    let entropy = b"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f";

    b.iter(|| {
        let _ = Mnemonic::from_entropy_in(LANG, &entropy[..]).unwrap();
    });
}

#[cfg(feature = "rand")]
#[bench]
fn new_mnemonic(b: &mut Bencher) {
    b.iter(|| {
        let _ = Mnemonic::generate_in(LANG, 24);
    });
}

#[bench]
fn to_seed(b: &mut Bencher) {
    let entropy = b"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f";
    let m = Mnemonic::from_entropy_in(LANG, &entropy[..]).unwrap();

    b.iter(|| {
        let _ = m.to_seed("");
    });
}
