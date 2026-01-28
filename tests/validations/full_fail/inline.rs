use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = FullFail)]
struct Test {
	#[validate(inline(|x: &bool| *x, [], "custom message", "custom_code"))]
	#[validate(inline(|x: &bool, b: &Option<bool>| b.is_some_and(|c| c) || *x, [&self.b], "custom message 2", "custom_code_2"))]
	pub a: bool,
	#[validate(inline(|x: &bool| *x, [], "custom message", "custom_code"))]
	#[validate(inline(|x: &bool, a: &bool| *a && *x, [&self.a], "custom message 2", "custom_code_2"))]
	pub b: Option<bool>,
}

#[test]
fn should_validate_inlines() {
	let cases = [false];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
			"b" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
		});
	}
}
