use serde::Deserialize;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(naive_date("%Y-%m-%d"))]
	#[validate(naive_date("%Y-%m-%d"))]
	pub a: String,
	#[validate(naive_date("%Y-%m-%d", "custom message"))]
	#[validate(naive_date("%Y-%m-%d", "custom message"))]
	pub b: Option<String>,
	#[validate(naive_date("%Y-%m-%d", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(naive_date("%Y-%m-%d", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_naive_dates() {
	let cases = [
		("2024-02-29", true),
		("2026-02-29", false),
		("1999-12-31", true),
		("2023-01-01", true),
		("10-07-2023", false),
		("2023-02-30", false),
		("random string", false),
		("", false),
		("14:30:00", false),
		("2023-07-10", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("naive_date", "invalid naive date format"),
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
				"b" => ("naive_date", "custom message"),
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
				"c" => ("custom_code", "invalid naive date format"),
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
