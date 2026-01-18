use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(contains("test"))]
	#[validate(contains("test"))]
	pub a: String,
	#[validate(contains("test", "custom message"))]
	#[validate(contains("test", "custom message"))]
	pub b: Option<String>,
	#[validate(contains("test", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(contains("test", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_contains() {
	let cases = [
		("example", false),
		("exampletest", true),
		("exampletestexample", true),
		("teste", true),
		("test", true),
		("est", false),
		("test example", true),
		("example test", true),
		("example test example", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("slice", "invalid format"),
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
				"b" => ("slice", "custom message"),
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
				"c" => ("custom_code", "invalid format"),
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
