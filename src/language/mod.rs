use std::fmt;

#[cfg(feature = "chinese-simplified")]
mod chinese_simplified;
#[cfg(feature = "chinese-traditional")]
mod chinese_traditional;
#[cfg(feature = "czech")]
mod czech;
mod english;
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

/// Static bidirectional map between words and indices
pub struct WordMap {
    ord_words: &'static [&'static str; 2048],
    word_idx: &'static [u16; 2048],
    rev_idx: &'static [u16; 2048],
}

impl WordMap {
    /// Get the index of the word in the word list. Returns `None` if not found.
    pub fn word_to_idx(&self, word: &str) -> Option<u16> {
        self.ord_words
            .binary_search(&word)
            .ok()
            .map(|pos| self.word_idx[pos])
    }

    /// Returns the `idx`-th word from the word list
    ///
    /// May panic if `idx >= 2048`
    pub fn idx_to_word(&self, idx: u16) -> &'static str {
        self.ord_words[self.rev_idx[idx as usize] as usize]
    }

    /// Returns all words from the word list beginning with `prefix`
    pub fn words_by_prefix(&self, prefix: &str) -> &[&'static str] {
        let first = match self.ord_words.iter().position(|w| w.starts_with(prefix)) {
            Some(i) => i,
            None => return &[],
        };
        let count = self.ord_words[first..]
            .iter()
            .take_while(|w| w.starts_with(prefix))
            .count();
        &self.ord_words[first..first + count]
    }
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

    /// The word list for this language.
    #[inline]
    pub(crate) fn word_map(self) -> WordMap {
        match self {
            Language::English => WordMap {
                ord_words: &english::WORDS,
                word_idx: &english::INDICES,
                rev_idx: &english::REV_INDICES,
            },
            #[cfg(feature = "chinese-simplified")]
            Language::SimplifiedChinese => WordMap {
                ord_words: &chinese_simplified::WORDS,
                word_idx: &chinese_simplified::INDICES,
                rev_idx: &chinese_simplified::REV_INDICES,
            },
            #[cfg(feature = "chinese-traditional")]
            Language::TraditionalChinese => WordMap {
                ord_words: &chinese_traditional::WORDS,
                word_idx: &chinese_traditional::INDICES,
                rev_idx: &chinese_traditional::REV_INDICES,
            },
            #[cfg(feature = "czech")]
            Language::Czech => WordMap {
                ord_words: &czech::WORDS,
                word_idx: &czech::INDICES,
                rev_idx: &czech::REV_INDICES,
            },
            #[cfg(feature = "french")]
            Language::French => WordMap {
                ord_words: &french::WORDS,
                word_idx: &french::INDICES,
                rev_idx: &french::REV_INDICES,
            },
            #[cfg(feature = "italian")]
            Language::Italian => WordMap {
                ord_words: &italian::WORDS,
                word_idx: &italian::INDICES,
                rev_idx: &italian::REV_INDICES,
            },
            #[cfg(feature = "japanese")]
            Language::Japanese => WordMap {
                ord_words: &japanese::WORDS,
                word_idx: &japanese::INDICES,
                rev_idx: &japanese::REV_INDICES,
            },
            #[cfg(feature = "korean")]
            Language::Korean => WordMap {
                ord_words: &korean::WORDS,
                word_idx: &korean::INDICES,
                rev_idx: &korean::REV_INDICES,
            },
            #[cfg(feature = "spanish")]
            Language::Spanish => WordMap {
                ord_words: &spanish::WORDS,
                word_idx: &spanish::INDICES,
                rev_idx: &spanish::REV_INDICES,
            },
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
        feature = "chinese-simplified",
        feature = "chinese-traditional",
        feature = "czech",
        feature = "french",
        feature = "italian",
        feature = "japanese",
        feature = "korean",
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

        use bitcoin_hashes::{sha256, Hash};
        use std::io::Write;

        let checksums = [
            (
                "5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726",
                Language::SimplifiedChinese,
            ),
            (
                "417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f",
                Language::TraditionalChinese,
            ),
            (
                "7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc",
                Language::Czech,
            ),
            (
                "2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda",
                Language::English,
            ),
            (
                "ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59",
                Language::French,
            ),
            (
                "d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2",
                Language::Italian,
            ),
            (
                "2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd",
                Language::Japanese,
            ),
            (
                "9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60",
                Language::Korean,
            ),
            (
                "46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b",
                Language::Spanish,
            ),
        ];

        for &(sum, lang) in &checksums {
            let mut digest = sha256::Hash::engine();
            for idx in 0..2048 {
                let word = lang.word_map().idx_to_word(idx);
                assert!(::unicode_normalization::is_nfkd(&word));
                write!(&mut digest, "{}\n", word).unwrap();
                assert_eq!(idx, lang.word_map().word_to_idx(word).unwrap());
            }
            assert_eq!(
                &sha256::Hash::from_engine(digest).to_string(),
                sum,
                "word list for language {} failed checksum check",
                lang,
            );
        }
    }

    #[test]
    fn words_by_prefix() {
        let lang = Language::English;
        let map = lang.word_map();

        let res = map.words_by_prefix("woo");
        assert_eq!(res, ["wood", "wool"]);

        let res = map.words_by_prefix("");
        assert_eq!(res.len(), 2048);

        let res = map.words_by_prefix("woof");
        assert!(res.is_empty());
    }
}
