use serde::Deserialize;
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = FailFast)]
struct Test {
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message", "custom_code"))]
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message 2", "custom_code_2"))]
	pub a: String,
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message", "custom_code"))]
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message 2", "custom_code_2"))]
	pub b: Option<String>,
	#[validate(blocklist("COLLECTION", ["a", "b"], "custom message", "custom_code"))]
	#[validate(blocklist("COLLECTION", ["a", "b"], "custom message 2", "custom_code_2"))]
	pub c: Vec<String>,
	#[validate(blocklist("COLLECTION", ["a", "b"], "custom message", "custom_code"))]
	#[validate(blocklist("COLLECTION", ["a", "b"], "custom message 2", "custom_code_2"))]
	pub d: Option<Vec<String>>,
}

#[test]
fn should_validate_blocklists() {
	let cases = (["a"], [vec!["a".into(), "c".into()]]);

	let mut test = Test::default();
	for case in cases.0.iter() {
		test.a = case.to_string();
		test.b = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code", "custom message"),
		});
	}

	for case in cases.1.iter() {
		test.c = case.clone();
		test.d = Some(case.clone());
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code", "custom message"),
		});
	}
}
