use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = FullFail)]
struct Test {
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message", "custom_code"))]
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message", "custom_code"))]
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
}

#[test]
fn should_validate_times() {
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
