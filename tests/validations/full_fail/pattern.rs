use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = FullFail)]
struct Test {
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message", "custom_code"))]
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message", "custom_code"))]
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
}

#[test]
fn should_validate_patterns() {
	let cases = [""];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = case.to_string();
		test.b = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
			"b" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
		});
	}
}
