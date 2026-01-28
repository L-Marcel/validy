use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(url("custom message", "custom_code"))]
	#[validate(url("custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(url("custom message", "custom_code"))]
	#[validate(url("custom message 2", "custom_code_2"))]
	pub b: Option<String>,
}

#[test]
fn should_validate_urls() {
	let cases = ["http://"];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = case.to_string();
		test.b = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
