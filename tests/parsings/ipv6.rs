use std::net::Ipv6Addr;
use std::str::FromStr;

use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(ipv6)]
	pub a: Ipv6Addr,
	#[special(from_type(String))]
	#[parse(ipv6)]
	pub b: Option<Ipv6Addr>,
}

#[test]
fn should_parse_ipv6s() {
	let cases = [
		("::1", Ipv6Addr::from_str("::1").expect("should be a valid ip")),
		("::", Ipv6Addr::from_str("::").expect("should be a valid ip")),
		(
			"2001:db8::1",
			Ipv6Addr::from_str("2001:db8::1").expect("should be a valid ip"),
		),
		(
			"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
			Ipv6Addr::from_str("2001:0db8:85a3:0000:0000:8a2e:0370:7334").expect("should be a valid ip"),
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

	wrapper.a = Some("invalid-ipv6".to_string());
	result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("ipv6", "invalid ipv6 format"),
	});
}
