use serde::Deserialize;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(time("%Y-%m-%d %H:%M:%S %z"))]
	#[validate(time("%Y-%m-%d %H:%M:%S %z"))]
	pub a: String,
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message"))]
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message"))]
	pub b: Option<String>,
	#[validate(time("%Y-%m-%d %H:%M:%S %z", code = "custom_code"))]
	pub c: Option<String>,
	#[validate(time("%Y-%m-%d %H:%M:%S %z", "custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_times() {
	let cases = [
		("2024-02-29 10:00:00 -0300", true),
		("1999-12-31 23:59:59 +0530", true),
		("2023-01-01 00:00:00 -0000", true),
		("2023-07-10 14:30:00", false),
		("10-07-2023 14:30:00 +0000", false),
		("2023-02-30 10:00:00 +0000", false),
		("2023-07-10 25:00:00 +0000", false),
		("random string", false),
		("", false),
		("2023-07-10", false),
		("14:30:00 +0000", false),
		("2023-07-10 14:30:00 +0000", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("time", "invalid time format"),
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
				"b" => ("time", "custom message"),
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
				"c" => ("custom_code", "invalid time format"),
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
