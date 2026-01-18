use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$"))]
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$"))]
	pub a: String,
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message"))]
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message"))]
	pub b: Option<String>,
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(pattern(r"^[A-Z]{3}-\d{3}$", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_patterns() {
	let cases = [
		("XYZ-000", true),
		("BRA-999", true),
		("abc-123", false),
		("ABC123", false),
		("AB-123", false),
		("ABCD-123", false),
		("ABC-12", false),
		("ABC-1234", false),
		("123-ABC", false),
		("", false),
		("ABC-123", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("pattern", "outside the accepted pattern"),
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
				"b" => ("pattern", "custom message"),
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
				"c" => ("custom_code", "outside the accepted pattern"),
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
