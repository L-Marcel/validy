use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(suffix("test"))]
	#[validate(suffix("test"))]
	pub a: String,
	#[validate(suffix("test", "custom message"))]
	#[validate(suffix("test", "custom message"))]
	pub b: Option<String>,
	#[validate(suffix("test", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(suffix("test", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_suffixes() {
	let cases = [
		("example", false),
		("exampletestexample", false),
		("teste", false),
		("test", true),
		("est", false),
		("test example", false),
		("example test", true),
		("example test example", false),
		("exampletest", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("suffix", "invalid suffix"),
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
				"b" => ("suffix", "custom message"),
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
				"c" => ("custom_code", "invalid suffix"),
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
