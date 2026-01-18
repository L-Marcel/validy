use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(ip)]
	#[validate(ip)]
	pub a: String,
	#[validate(ip("custom message"))]
	#[validate(ip("custom message"))]
	pub b: Option<String>,
	#[validate(ip(code = "custom_code"))]
	pub c: Option<String>,
	#[validate(ip("custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_ips() {
	let cases = [
		("0.0.0.0", true),
		("255.255.255.255", true),
		("192.168.0.1", true),
		("::1", true),
		("::", true),
		("2001:db8::1", true),
		("2001:0db8:85a3:0000:0000:8a2e:0370:7334", true),
		("", false),
		("test", false),
		("google.com", false),
		("192.168.1. 1", false),
		("256.256.256.256", false),
		("1.2.3", false),
		("1.2.3.4.5", false),
		("1200::AB00::1", false),
		("g::1", false),
		("127.0.0.1", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("ip", "invalid ip format"),
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
				"b" => ("ip", "custom message"),
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
				"c" => ("custom_code", "invalid ip format"),
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
