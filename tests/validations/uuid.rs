use serde::Deserialize;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(uuid)]
	#[validate(uuid)]
	pub a: String,
	#[validate(uuid("custom message"))]
	#[validate(uuid("custom message"))]
	pub b: Option<String>,
	#[validate(uuid(code = "custom_code"))]
	pub c: Option<String>,
	#[validate(uuid("custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_uuids() {
	let cases = [
		("urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8", true),
		("00000000-0000-0000-0000-000000000000", true),
		("invalid-uuid", false),
		("f47ac10b-58cc-4372-a567-0e02b2c3d47", false),
		("", false),
		("f47ac10b-58cc-4372-a567-0e02b2c3d479", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("uuid", "invalid uuid format"),
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
				"b" => ("uuid", "custom message"),
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
				"c" => ("custom_code", "invalid uuid format"),
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
