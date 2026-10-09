#![cfg(feature = "async")]

use std::cell::Cell;
use std::future::{Future, poll_fn};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use bip39::{Language, Mnemonic};

async fn yield_once() {
	let mut yielded = false;
	poll_fn(|cx| {
		if yielded {
			Poll::Ready(())
		} else {
			yielded = true;
			cx.waker().wake_by_ref();
			Poll::Pending
		}
	})
	.await
}

fn finish(future: impl Future<Output = [u8; 64]>, expected_yields: usize) -> [u8; 64] {
	let mut future = pin!(future);
	let mut cx = Context::from_waker(Waker::noop());
	for _ in 0..expected_yields {
		assert!(future.as_mut().poll(&mut cx).is_pending());
	}
	match future.as_mut().poll(&mut cx) {
		Poll::Ready(seed) => seed,
		Poll::Pending => panic!("derivation did not finish"),
	}
}

#[test]
fn test_to_seed_normalized_async() {
	let passphrases = [
		String::new(),
		"TREZOR".to_owned(),
		"pa\u{308}ssphrase".to_owned(),
		"\0x".to_owned(),
		"p".repeat(512),
	];
	for &language in Language::ALL {
		for (i, &len) in [16, 20, 24, 28, 32].iter().enumerate() {
			let entropy: Vec<u8> = (0..len).map(|n| (n * 37 + i) as u8).collect();
			let mnemonic = Mnemonic::from_entropy_in(language, &entropy).unwrap();
			let passphrase = &passphrases[i];
			let expected = mnemonic.to_seed_normalized(passphrase);
			assert_eq!(
				finish(mnemonic.to_seed_normalized_async(passphrase, yield_once), 2048),
				expected
			);
			assert_eq!(
				finish(mnemonic.to_seed_normalized_async(passphrase, || async {}), 0),
				expected
			);
		}
	}
}

#[test]
fn test_to_seed_normalized_async_cancellation() {
	struct OnDrop<'a>(&'a Cell<usize>);
	impl Drop for OnDrop<'_> {
		fn drop(&mut self) {
			self.0.set(self.0.get() + 1);
		}
	}

	let mnemonic = Mnemonic::from_entropy(&[0; 16]).unwrap();
	for polls in [0, 1, 2048].iter().copied() {
		let drops = Cell::new(0);
		let started = Cell::new(0);
		let finished = Cell::new(0);
		let mut future = Box::pin(mnemonic.to_seed_normalized_async("", || async {
			let _guard = OnDrop(&drops);
			started.set(started.get() + 1);
			yield_once().await;
			finished.set(finished.get() + 1);
		}));
		let mut cx = Context::from_waker(Waker::noop());
		for _ in 0..polls {
			assert!(future.as_mut().poll(&mut cx).is_pending());
		}
		drop(future);
		assert_eq!(started.get(), polls);
		assert_eq!(drops.get(), polls);
		assert_eq!(finished.get(), polls.saturating_sub(1));
	}
}
