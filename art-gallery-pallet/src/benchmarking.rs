#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use sp_runtime::traits::Bounded;

use crate::Module as Gallery;


benchmarks! {
	create_collection {
	}: create_collection(RawOrigin::Signed(whitelisted_caller().clone()))
	verify {
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests_composite::{ExtBuilder, Test};
	use frame_support::assert_ok;

	#[test]
	fn create_collection() {
		ExtBuilder::default().build().execute_with(|| {
			assert_ok!(test_benchmark_create_collection::<Test>());
		});
	}
}
