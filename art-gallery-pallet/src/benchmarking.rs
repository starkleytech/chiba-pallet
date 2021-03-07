#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use sp_runtime::traits::Bounded;

use crate::Module as Gallery;

fn last_event() -> crate::mock::Event {
  frame_system::Module::<crate::mock::Test>::events().pop().expect("Event expected").event
}

benchmarks! {
	create_collection {
    let caller: T::AccountId = whitelisted_caller();
	}: create_collection(RawOrigin::Signed(caller.clone()), Vec::<u8>::default(), ClassData::default())
	verify {
    assert_eq!(
      last_event(),
      crate::mock::Event::pallet_gallery(crate::RawEvent::CollectionCreated(0)),
    );
	}

	mint {
    let caller: T::AccountId = whitelisted_caller();
	}: mint(RawOrigin::Signed(caller.clone()), Default::default(), Vec::<u8>::default(), TokenData::default())
	verify {
    assert_eq!(
      last_event(),
      crate::mock::Event::pallet_gallery(crate::RawEvent::CollectionCreated(0)),
    );
	}
}


#[cfg(test)]
mod tests {
	use super::*;
  use crate::mock::*;
	use frame_support::assert_ok;

	#[test]
	fn create_collection() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_create_collection::<Test>());
		});
	}
}
