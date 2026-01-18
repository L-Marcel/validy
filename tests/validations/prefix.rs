use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(prefix("test"))]
	#[validate(prefix("test"))]
	pub a: String,
	#[validate(prefix("test", "custom message"))]
	#[validate(prefix("test", "custom message"))]
	pub b: Option<String>,
	#[validate(prefix("test", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(prefix("test", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_prefixes() {
	let cases = [
		("example", false),
		("exampletest", false),
		("exampletestexample", false),
		("teste", true),
		("est", false),
		("test example", true),
		("example test", false),
		("example test example", false),
		("test", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("prefix", "invalid prefix"),
			});
		}
	}

	for (case, is_valid) in cases.iter() {
		test.b = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("prefix", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.iter() {
		test.c = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "invalid prefix"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.iter() {
		test.d = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"d" => ("custom_code", "custom message"),
			});
		}
	}
}
