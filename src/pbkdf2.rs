
use bitcoin_hashes::{hmac, sha512, Hash, HashEngine};

// Method borrowed from rust-bitcoin's endian module.
#[inline]
fn u32_to_array_be(val: u32) -> [u8; 4] {
	debug_assert_eq!(::std::mem::size_of::<u32>(), 4); // size_of isn't a constfn in 1.22

	let mut res = [0; 4];
	for i in 0..4 {
		res[i] = ((val >> (4 - i - 1)*8) & 0xff) as u8;
	}
	res
}

#[inline]
fn xor(res: &mut [u8], salt: &[u8]) {
	debug_assert!(salt.len() >= res.len(), "length mismatch in xor");

	res.iter_mut().zip(salt.iter()).for_each(|(a, b)| *a ^= b);
}

/// PBKDF2-HMAC-SHA512 implementation using bitcoin_hashes.
pub(crate) fn pbkdf2(passphrase: &[u8], salt: &[u8], c: usize, res: &mut [u8]) {
	let prf = hmac::HmacEngine::<sha512::Hash>::new(passphrase);

	for (i, chunk) in res.chunks_mut(sha512::Hash::LEN).enumerate() {
		for v in chunk.iter_mut() { *v = 0; }

		let mut salt = {
			let mut prfc = prf.clone();
			prfc.input(salt);
			prfc.input(&u32_to_array_be((i + 1) as u32));

			let salt = hmac::Hmac::from_engine(prfc).into_inner();
			xor(chunk, &salt);
			salt
		};

		for _ in 1..c {
			let mut prfc = prf.clone();
			prfc.input(&salt);
			salt = hmac::Hmac::from_engine(prfc).into_inner();

			xor(chunk, &salt);
		}
	}
}
