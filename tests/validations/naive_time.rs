use serde::Deserialize;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(naive_time("%Y-%m-%d %H:%M:%S"))]
	#[validate(naive_time("%Y-%m-%d %H:%M:%S"))]
	pub a: String,
	#[validate(naive_time("%Y-%m-%d %H:%M:%S", "custom message"))]
	#[validate(naive_time("%Y-%m-%d %H:%M:%S", "custom message"))]
	pub b: Option<String>,
	#[validate(naive_time("%Y-%m-%d %H:%M:%S", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(naive_time("%Y-%m-%d %H:%M:%S", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_naive_times() {
	let cases = [
		("2024-02-29 10:00:00", true),
		("1999-12-31 23:59:59", true),
		("2023-01-01 00:00:00", true),
		("2023-07-10 14:30:00", true),
		("10-07-2023 14:30:00", false),
		("2023-02-30 10:00:00", false),
		("2023-07-10 25:00:00", false),
		("random string", false),
		("", false),
		("2023-07-10", false),
		("14:30:00", false),
		("2023-07-10 14:30:00", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("naive_time", "invalid naive time format"),
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
				"b" => ("naive_time", "custom message"),
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
				"c" => ("custom_code", "invalid naive time format"),
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
