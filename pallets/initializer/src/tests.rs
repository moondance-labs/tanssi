use super::*;
use crate::mock::{new_test_ext, session_change_validators, Initializer, System, Test};

use frame_support::traits::{OnFinalize, OnInitialize};

#[test]
fn session_0_is_instantly_applied() {
	new_test_ext().execute_with(|| {
		Initializer::test_trigger_on_new_session(
			false,
			0,
			Vec::new().into_iter(),
			Some(Vec::new().into_iter()),
		);

		let v = BufferedSessionChanges::<Test>::get();
		assert!(!v.is_some());

		assert_eq!(session_change_validators(), Some((0, Vec::new())));
	});
}

#[test]
fn session_change_before_initialize_is_still_buffered_after() {
	new_test_ext().execute_with(|| {
		Initializer::test_trigger_on_new_session(
			false,
			1,
			Vec::new().into_iter(),
			Some(Vec::new().into_iter()),
		);

		let now = System::block_number();
		Initializer::on_initialize(now);

		// Session change validators are applied after on_finalize
		assert_eq!(session_change_validators(), None);

		let v = BufferedSessionChanges::<Test>::get();
		assert!(v.is_some());
	});
}

#[test]
fn session_change_applied_on_finalize() {
	new_test_ext().execute_with(|| {
		Initializer::on_initialize(1);
		Initializer::test_trigger_on_new_session(
			false,
			1,
			Vec::new().into_iter(),
			Some(Vec::new().into_iter()),
		);

		Initializer::on_finalize(1);

		// Session change validators are applied after on_finalize
		assert_eq!(session_change_validators(), Some((1, Vec::new())));

		assert!(!BufferedSessionChanges::<Test>::get().is_some());
	});
}
