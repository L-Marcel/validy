use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(length(0..5, "custom message", "custom_code"))]
	#[validate(length(0..5, "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(length(0..5, "custom message", "custom_code"))]
	#[validate(length(0..5, "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
	#[validate(length(0..5, "custom message", "custom_code"))]
	#[validate(length(0..5, "custom message 2", "custom_code_2"))]
	pub c: Vec<String>,
	#[validate(length(0..5, "custom message", "custom_code"))]
	#[validate(length(0..5, "custom message 2", "custom_code_2"))]
	pub d: Option<Vec<String>>,
}

#[test]
fn should_validate_lengths() {
	let cases = (["abcde"], [vec!["a".to_string(); 5]]);

	let mut test = Test::default();
	for case in cases.0.iter() {
		test.a = case.to_string();
		test.b = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}

	for case in cases.1.iter() {
		test.c = case.clone();
		test.d = Some(case.clone());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
			"c" => ("custom_code_2", "custom message 2"),
			"d" => ("custom_code_2", "custom message 2"),
		});
	}
}
