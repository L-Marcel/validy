use std::net::Ipv4Addr;
use std::str::FromStr;

use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(ipv4)]
	pub a: Ipv4Addr,
	#[special(from_type(String))]
	#[parse(ipv4)]
	pub b: Option<Ipv4Addr>,
}

#[test]
fn should_parse_ipv4s() {
	let cases = [
		("0.0.0.0", Ipv4Addr::from_str("0.0.0.0").expect("should be a valid ip")),
		(
			"255.255.255.255",
			Ipv4Addr::from_str("255.255.255.255").expect("should be a valid ip"),
		),
		(
			"192.168.0.1",
			Ipv4Addr::from_str("192.168.0.1").expect("should be a valid ip"),
		),
		(
			"127.0.0.1",
			Ipv4Addr::from_str("127.0.0.1").expect("should be a valid ip"),
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

	wrapper.a = Some("invalid-ipv4".to_string());
	result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("ipv4", "invalid ipv4 format"),
	});
}
