#[cfg(feature = "japanese")]
mod japanese_tests {
    use bip39::{Mnemonic, Language};
    use bitcoin_hashes::hex::FromHex;

    #[test]
    fn test_japanese_vectors() {
        let test_vectors = [
            (
                "00000000000000000000000000000000",
                "あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あおぞら",
                "メートルガバヴァぱばぐゞちぢ十人十色",
                "a262d6fb6122ecf45be09c50492b31f92e9beb7d9a845987a02cefda57a15f9c467a17872029a9e92299b5cbdf306e3a0ee620245cbd508959b6cb7ca637bd55",
            ),
            (
                "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
                "そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れきだい　ほんやく　わかめ",
                "メートルガバヴァぱばぐゞちぢ十人十色",
                "aee025cbe6ca256862f889e48110a6a382365142f7d16f2b9545285b3af64e542143a577e9c144e101a6bdca18f8d97ec3366ebf5b088b1c1af9bc31346e60d9",
            ),
            (
                "80808080808080808080808080808080",
                "そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　あまど　おおう　あかちゃん",
                "メートルガバヴァぱばぐゞちぢ十人十色",
                "e51736736ebdf77eda23fa17e31475fa1d9509c78f1deb6b4aacfbd760a7e2ad769c714352c95143b5c1241985bcb407df36d64e75dd5a2b78ca5d2ba82a3544",
            ),
            (
                "ffffffffffffffffffffffffffffffff",
                "われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　ろんぶん",
                "メートルガバヴァぱばぐゞちぢ十人十色",
                "4cd2ef49b479af5e1efbbd1e0bdc117f6a29b1010211df4f78e2ed40082865793e57949236c43b9fe591ec70e5bb4298b8b71dc4b267bb96ed4ed282c8f7761c",
            ),
            (
                "77c2b00716cec7213839159e404db50d",
                "せまい　うちがわ　あずき　かろう　めずらしい　だんち　ますく　おさめる　ていぼう　あたる　すあな　えしゃく",
                "メートルガバヴァぱばぐゞちぢ十人十色",
                "344cef9efc37d0cb36d89def03d09144dd51167923487eec42c487f7428908546fa31a3c26b7391a2b3afe7db81b9f8c5007336b58e269ea0bd10749a87e0193",
            ),
        ];

        for (entropy_hex, expected_mnemonic, passphrase, expected_seed_hex) in test_vectors.iter() {            
            let entropy = Vec::from_hex(entropy_hex).unwrap();            
            let mnemonic = Mnemonic::from_entropy_in(Language::Japanese, &entropy).unwrap();
            let generated_mnemonic = mnemonic.to_string();
            
            // Test ideographic spaces in generated mnemonic
            assert!(generated_mnemonic.contains('　'), "Generated mnemonic should use ideographic spaces");
            
            let parsed_expected = Mnemonic::parse_in(Language::Japanese, *expected_mnemonic).unwrap();
            assert_eq!(parsed_expected.to_entropy(), entropy, "Expected mnemonic should parse back to original entropy");
            
            // Test seed generation with expected mnemonic (using to_seed to handle normalization)
            let seed = parsed_expected.to_seed(*passphrase);
            let expected_seed = Vec::from_hex(expected_seed_hex).unwrap();
            assert_eq!(seed.to_vec(), expected_seed, "Seed should match expected for entropy {}", entropy_hex);
        }
    }
}
