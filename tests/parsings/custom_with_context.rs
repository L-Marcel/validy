use validy::core::{ParseResult, Validate, ValidateAndParseWithContext};
use validy::{assert_errors, assert_parsed, validation_error};

#[allow(unused)]
#[derive(Debug, Validate, PartialEq)]
#[validate(payload, context = bool)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(custom_with_context(parse))]
	pub a: u32,
	#[special(from_type(String))]
	#[parse(custom_with_context(parse_two, [&tmp_2_a]))]
	pub b: Option<u32>,
	#[special(from_type(String))]
	#[parse(custom_with_context(parse_three, [&tmp_2_a, &tmp_2_b]))]
	pub c: Option<u32>,
}

fn parse(value: String, _field_name: &str, context: &bool) -> ParseResult<u32> {
	if *context {
		(value.parse::<u32>().unwrap_or(0), None)
	} else {
		(0, None)
	}
}

fn parse_two(value: String, _field_name: &str, context: &bool, extra_arg: &Option<u32>) -> ParseResult<u32> {
	if *context {
		(extra_arg.unwrap_or(value.parse::<u32>().unwrap_or(0)), None)
	} else {
		(1, None)
	}
}

fn parse_three(_: String, field_name: &str, _context: &bool, a: &Option<u32>, b: &Option<u32>) -> ParseResult<u32> {
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
fn should_parse_customs_with_context() {
	let cases = [("4", false, 0), ("a", true, 0), ("8", true, 8)];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse_with_context(wrapper.clone(), &true);
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, context, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse_with_context(wrapper.clone(), context);

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

	for (case, context, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse_with_context(wrapper.clone(), context);

		let expected = if *context {
			Test {
				a: *expected,
				b: Some(*expected),
				c: None,
			}
		} else {
			Test {
				a: *expected,
				b: Some(1),
				c: None,
			}
		};

		assert_parsed!(result, wrapper, expected);
	}

	wrapper.c = Some("".to_string());
	wrapper.b = None;
	result = Test::validate_and_parse_with_context(wrapper.clone(), &true);
	assert_errors!(result, wrapper, {
	  "c" => ("custom_code", "custom message")
	});

	for (case, context, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		wrapper.c = Some(case.to_string());
		result = Test::validate_and_parse_with_context(wrapper.clone(), context);

		let expected = if *context {
			Test {
				a: *expected,
				b: Some(*expected),
				c: Some(*expected),
			}
		} else {
			Test {
				a: *expected,
				b: Some(1),
				c: Some(*expected),
			}
		};

		assert_parsed!(result, wrapper, expected);
	}
}
