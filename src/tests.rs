use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(Chiba::create_collection(
            Origin::signed(1),
            Vec::new(),
            crate::ClassData::default()
        ));
    });
}
