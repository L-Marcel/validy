use chrono::NaiveDate;
use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(naive_date("%Y-%m-%d"))]
	pub a: NaiveDate,
	#[special(from_type(String))]
	#[parse(naive_date("%Y-%m-%d"))]
	pub b: Option<NaiveDate>,
}

#[test]
fn should_parse_naive_dates() {
	let cases = [
		(
			"2024-02-29",
			NaiveDate::parse_from_str("2024-02-29", "%Y-%m-%d").expect("should be a valid naive date"),
		),
		(
			"1999-12-31",
			NaiveDate::parse_from_str("1999-12-31", "%Y-%m-%d").expect("should be a valid naive date"),
		),
		(
			"2023-01-01",
			NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").expect("should be a valid naive date"),
		),
		(
			"2023-07-10",
			NaiveDate::parse_from_str("2023-07-10", "%Y-%m-%d").expect("should be a valid naive date"),
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
