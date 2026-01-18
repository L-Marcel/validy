use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(ipv4)]
	#[validate(ipv4)]
	pub a: String,
	#[validate(ipv4("custom message"))]
	#[validate(ipv4("custom message"))]
	pub b: Option<String>,
	#[validate(ipv4(code = "custom_code"))]
	pub c: Option<String>,
	#[validate(ipv4("custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_ipv4s() {
	let cases = [
		("0.0.0.0", true),
		("255.255.255.255", true),
		("192.168.0.1", true),
		("::1", false),
		("::", false),
		("2001:db8::1", false),
		("2001:0db8:85a3:0000:0000:8a2e:0370:7334", false),
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
				"a" => ("ipv4", "invalid ipv4 format"),
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
				"b" => ("ipv4", "custom message"),
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
				"c" => ("custom_code", "invalid ipv4 format"),
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
