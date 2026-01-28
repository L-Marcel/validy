use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(contains("test", "custom message", "custom_code"))]
	#[validate(contains("test", "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(contains("test", "custom message", "custom_code"))]
	#[validate(contains("test", "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
}

#[test]
fn should_validate_contains() {
	let cases = ["example"];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = case.to_string();
		test.b = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code", "custom message"),
		  "b" => ("custom_code", "custom message"),
		});
	}
}
