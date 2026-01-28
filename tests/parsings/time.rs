use chrono::{DateTime, FixedOffset};
use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(time("%Y-%m-%d %H:%M:%S %z"))]
	pub a: DateTime<FixedOffset>,
	#[special(from_type(String))]
	#[parse(time("%Y-%m-%d %H:%M:%S %z"))]
	pub b: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_parse_times() {
	let cases = [
		(
			"2024-02-29 10:00:00 -0300",
			DateTime::parse_from_str("2024-02-29 10:00:00 -0300", "%Y-%m-%d %H:%M:%S %z")
				.expect("should be a valid naive date"),
		),
		(
			"1999-12-31 23:59:59 +0530",
			DateTime::parse_from_str("1999-12-31 23:59:59 +0530", "%Y-%m-%d %H:%M:%S %z")
				.expect("should be a valid naive date"),
		),
		(
			"2023-01-01 00:00:00 -0000",
			DateTime::parse_from_str("2023-01-01 00:00:00 -0000", "%Y-%m-%d %H:%M:%S %z")
				.expect("should be a valid naive date"),
		),
		(
			"2023-07-10 14:30:00 +0000",
			DateTime::parse_from_str("2023-07-10 14:30:00 +0000", "%Y-%m-%d %H:%M:%S %z")
				.expect("should be a valid naive date"),
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
}
