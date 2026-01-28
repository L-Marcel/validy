use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(prefix("test", "custom message", "custom_code"))]
	#[validate(prefix("test", "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(prefix("test", "custom message", "custom_code"))]
	#[validate(prefix("test", "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
}

#[test]
fn should_validate_prefixes() {
	let cases = [""];

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
