use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(range(0..5, "custom message", "custom_code"))]
	#[validate(range(0..5, "custom message 2", "custom_code_2"))]
	pub a: u8,
	#[validate(range(0..5, "custom message", "custom_code"))]
	#[validate(range(0..5, "custom message 2", "custom_code_2"))]
	pub b: Option<u8>,
}

#[test]
fn should_validate_ranges() {
	let cases = [6];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
