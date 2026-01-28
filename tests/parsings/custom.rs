use validy::core::{ParseResult, Validate, ValidateAndParse};
use validy::{assert_errors, assert_parsed, validation_error};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(custom(parse))]
	pub a: u32,
	#[special(from_type(String))]
	#[parse(custom(parse_two, [&tmp_2_a]))]
	pub b: Option<u32>,
	#[special(from_type(String))]
	#[parse(custom(parse_three, [&tmp_2_a, &tmp_2_b]))]
	pub c: Option<u32>,
}

fn parse(value: String, _field_name: &str) -> ParseResult<u32> {
	(value.parse::<u32>().unwrap_or(0), None)
}

fn parse_two(value: String, _field_name: &str, extra_arg: &Option<u32>) -> ParseResult<u32> {
	(extra_arg.unwrap_or(value.parse::<u32>().unwrap_or(0)), None)
}

fn parse_three(_: String, field_name: &str, a: &Option<u32>, b: &Option<u32>) -> ParseResult<u32> {
	match (a, b) {
		(_, None) => (
			0,
			Some(validation_error!(
				field_name.to_string(),
				"custom_code",
				"custom message"
			)),
		),
		(Some(a), _) => (*a, None),
		(None, Some(b)) => (*b, None),
	}
}

#[test]
fn should_parse_customs() {
	let cases = [("4", 4), ("a", 0), ("8", 8)];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: *expected,
				b: None,
				c: None,
			}
		);
	}

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: *expected,
				b: Some(*expected),
				c: None
			}
		);
	}

	wrapper.c = Some("".to_string());
	wrapper.b = None;
	result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
	  "c" => ("custom_code", "custom message")
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		wrapper.c = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: *expected,
				b: Some(*expected),
				c: Some(*expected)
			}
		);
	}
}
