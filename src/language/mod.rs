
use std::fmt;

mod english;
#[cfg(feature = "chinese-simplified")]
mod chinese_simplified;
#[cfg(feature = "chinese-traditional")]
mod chinese_traditional;
#[cfg(feature = "czech")]
mod czech;
#[cfg(feature = "french")]
mod french;
#[cfg(feature = "italian")]
mod italian;
#[cfg(feature = "japanese")]
mod japanese;
#[cfg(feature = "korean")]
mod korean;
#[cfg(feature = "spanish")]
mod spanish;

#[cfg(not(feature = "low-memory"))]
mod lazy {
	use std::cell::Cell;
	use std::collections::HashMap;
	use std::sync::Once;

	/// Type used to load a word map in a lazy fashion.
	pub(crate) struct LazyMap(Cell<Option<HashMap<&'static str, u16>>>, Once);

	impl LazyMap {
		#[allow(deprecated)]
		const INIT: Self = LazyMap(Cell::new(None), ::std::sync::ONCE_INIT);

		#[inline(always)]
		pub fn get(&'static self, list: &'static [&'static str]) -> &HashMap<&'static str, u16> {
			self.1.call_once(|| {
				let mut map = HashMap::new();
				for (idx, word) in list.iter().enumerate() {
					map.insert(*word, idx as u16);
				}
				self.0.set(Some(map));
			});

			// `self.0` is guaranteed to be `Some` by this point
			// The `Once` will catch and propagate panics
			unsafe {
				match *self.0.as_ptr() {
					Some(ref x) => x,
					None => panic!(),
				}
			}
		}
	}

	// This marker impl is required for the Cell to work.
	// The LazyMap is an implementation identical to lazy_static's.
	// We assume lazy_static's exact same usage is considered safe.
	#[cfg(not(feature = "low-memory"))]
	unsafe impl Sync for LazyMap {}

	pub(crate) static LAZY_MAP_ENGLISH: LazyMap = LazyMap::INIT;
	#[cfg(feature = "chinese-simplified")]
	pub(crate) static LAZY_MAP_CHINESE_SIMPLIFIED: LazyMap = LazyMap::INIT;
	#[cfg(feature = "chinese-traditional")]
	pub(crate) static LAZY_MAP_CHINESE_TRADITIONAL: LazyMap = LazyMap::INIT;
	#[cfg(feature = "czech")]
	pub(crate) static LAZY_MAP_CZECH: LazyMap = LazyMap::INIT;
	#[cfg(feature = "french")]
	pub(crate) static LAZY_MAP_FRENCH: LazyMap = LazyMap::INIT;
	#[cfg(feature = "italian")]
	pub(crate) static LAZY_MAP_ITALIAN: LazyMap = LazyMap::INIT;
	#[cfg(feature = "japanese")]
	pub(crate) static LAZY_MAP_JAPANESE: LazyMap = LazyMap::INIT;
	#[cfg(feature = "korean")]
	pub(crate) static LAZY_MAP_KOREAN: LazyMap = LazyMap::INIT;
	#[cfg(feature = "spanish")]
	pub(crate) static LAZY_MAP_SPANISH: LazyMap = LazyMap::INIT;
}

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
	/// The English language.
	English,
	#[cfg(feature = "chinese-simplified")]
	/// The Simplified Chinese language.
	SimplifiedChinese,
	#[cfg(feature = "chinese-traditional")]
	/// The Traditional Chinese language.
	TraditionalChinese,
	#[cfg(feature = "czech")]
	/// The Czech language.
	Czech,
	#[cfg(feature = "french")]
	/// The French language.
	French,
	#[cfg(feature = "italian")]
	/// The Italian language.
	Italian,
	#[cfg(feature = "japanese")]
	/// The Japanese language.
	Japanese,
	#[cfg(feature = "korean")]
	/// The Korean language.
	Korean,
	#[cfg(feature = "spanish")]
	/// The Spanish language.
	Spanish,
}

impl Language {
	/// Get words from the wordlist that start with the given prefix.
	pub fn words_by_prefix(self, prefix: &str) -> &[&'static str] {
		let first = match self.word_list().iter().position(|w| w.starts_with(prefix)) {
			Some(i) => i,
			None => return &[],
		};
		let count = self.word_list()[first..].iter().take_while(|w| w.starts_with(prefix)).count();
		&self.word_list()[first .. first + count]
	}

	/// The word list for this language.
	#[inline]
	pub(crate) fn word_list(self) -> &'static [&'static str; 2048] {
		match self {
			Language::English => &english::WORDS,
			#[cfg(feature = "chinese-simplified")]
			Language::SimplifiedChinese => &chinese_simplified::WORDS,
			#[cfg(feature = "chinese-traditional")]
			Language::TraditionalChinese => &chinese_traditional::WORDS,
			#[cfg(feature = "czech")]
			Language::Czech => &czech::WORDS,
			#[cfg(feature = "french")]
			Language::French => &french::WORDS,
			#[cfg(feature = "italian")]
			Language::Italian => &italian::WORDS,
			#[cfg(feature = "japanese")]
			Language::Japanese => &japanese::WORDS,
			#[cfg(feature = "korean")]
			Language::Korean => &korean::WORDS,
			#[cfg(feature = "spanish")]
			Language::Spanish => &spanish::WORDS,
		}
	}

	/// The word map that maps words to the index in the word list for this language.
	#[cfg(not(feature = "low-memory"))]
	pub(crate) fn word_map(self) -> &'static ::std::collections::HashMap<&'static str, u16> {
		match self {
			Language::English => lazy::LAZY_MAP_ENGLISH.get(self.word_list()),
			#[cfg(feature = "chinese-simplified")]
			Language::SimplifiedChinese => lazy::LAZY_MAP_CHINESE_SIMPLIFIED.get(self.word_list()),
			#[cfg(feature = "chinese-traditional")]
			Language::TraditionalChinese => lazy::LAZY_MAP_CHINESE_TRADITIONAL.get(self.word_list()),
			#[cfg(feature = "czech")]
			Language::Czech => lazy::LAZY_MAP_CZECH.get(self.word_list()),
			#[cfg(feature = "french")]
			Language::French => lazy::LAZY_MAP_FRENCH.get(self.word_list()),
			#[cfg(feature = "italian")]
			Language::Italian => lazy::LAZY_MAP_ITALIAN.get(self.word_list()),
			#[cfg(feature = "japanese")]
			Language::Japanese => lazy::LAZY_MAP_JAPANESE.get(self.word_list()),
			#[cfg(feature = "korean")]
			Language::Korean => lazy::LAZY_MAP_KOREAN.get(self.word_list()),
			#[cfg(feature = "spanish")]
			Language::Spanish => lazy::LAZY_MAP_SPANISH.get(self.word_list()),
		}
	}
}

impl fmt::Display for Language {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(all(
		feature = "chinese-simplified", feature = "chinese-traditional", feature = "czech",
		feature = "french", feature = "italian", feature = "japanese", feature = "korean",
		feature = "spanish"
	))]
	#[test]
	fn validate_wordlist_checksums() {
		//! In this test, we ensure that the wordlists are identical.
		//!
		//! They are as follows in the bips repository:
		//! 5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726  chinese_simplified.txt
		//! 417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f  chinese_traditional.txt
		//! 7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc  czech.txt
		//! 2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda  english.txt
		//! ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59  french.txt
		//! d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2  italian.txt
		//! 2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd  japanese.txt
		//! 9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60  korean.txt
		//! 46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b  spanish.txt

		use std::io::Write;
		use bitcoin_hashes::{sha256, Hash};

		let checksums = [
			("5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726", Language::SimplifiedChinese),
			("417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f", Language::TraditionalChinese),
			("7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc", Language::Czech),
			("2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda", Language::English),
			("ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59", Language::French),
			("d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2", Language::Italian),
			("2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd", Language::Japanese),
			("9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60", Language::Korean),
			("46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b", Language::Spanish),
		];

		for &(sum, lang) in &checksums {
			let mut digest = sha256::Hash::engine();
			for (_idx, word) in lang.word_list().iter().enumerate() {
				assert!(::unicode_normalization::is_nfkd(&word));
				write!(&mut digest, "{}\n", word).unwrap();
				#[cfg(not(feature = "low-memory"))]
				assert_eq!(_idx, lang.word_map()[word] as usize);
			}
			assert_eq!(&sha256::Hash::from_engine(digest).to_string(), sum,
				"word list for language {} failed checksum check", lang,
			);
		}
	}

	#[test]
	fn words_by_prefix() {
		let lang = Language::English;

		let res = lang.words_by_prefix("woo");
		assert_eq!(res, ["wood","wool"]);

		let res = lang.words_by_prefix("");
		assert_eq!(res.len(), 2048);

		let res = lang.words_by_prefix("woof");
		assert!(res.is_empty());
	}
}
