use std::net::IpAddr;
use std::str::FromStr;

use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(ip)]
	pub a: IpAddr,
	#[special(from_type(String))]
	#[parse(ip)]
	pub b: Option<IpAddr>,
}

#[test]
fn should_parse_ips() {
	let cases = [
		("0.0.0.0", IpAddr::from_str("0.0.0.0").expect("should be a valid ip")),
		(
			"255.255.255.255",
			IpAddr::from_str("255.255.255.255").expect("should be a valid ip"),
		),
		(
			"192.168.0.1",
			IpAddr::from_str("192.168.0.1").expect("should be a valid ip"),
		),
		("::1", IpAddr::from_str("::1").expect("should be a valid ip")),
		("::", IpAddr::from_str("::").expect("should be a valid ip")),
		(
			"2001:db8::1",
			IpAddr::from_str("2001:db8::1").expect("should be a valid ip"),
		),
		(
			"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
			IpAddr::from_str("2001:0db8:85a3:0000:0000:8a2e:0370:7334").expect("should be a valid ip"),
		),
		(
			"127.0.0.1",
			IpAddr::from_str("127.0.0.1").expect("should be a valid ip"),
		),
	];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(result, wrapper, Test { a: *expected, b: None });
	}

	let last_a = result.expect("should be a valid result").a;
	for (case, expected) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a,
				b: Some(*expected)
			}
		);
	}

	wrapper.a = Some("invalid-ip".to_string());
	result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("ip", "invalid ip format"),
	});
}
