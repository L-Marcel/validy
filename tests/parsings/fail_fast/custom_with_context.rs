use validy::core::{ParseResult, Validate, ValidateAndParseWithContext};
use validy::{assert_errors, validation_error};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, context = bool, failure_mode = FailFast)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(custom_with_context(parse))]
	#[parse(custom_with_context(parse_two, [&wrapper.b]))]
	pub a: u32,
	#[special(from_type(String))]
	#[parse(custom_with_context(parse))]
	#[parse(custom_with_context(parse_three, [&tmp_3_a, &tmp_3_b]))]
	pub b: Option<u32>,
}

fn parse(_value: String, field: &str, _context: &bool) -> ParseResult<u32> {
	(
		0,
		Some(validation_error!(field.to_string(), "custom_code", "custom message")),
	)
}

fn parse_two(_value: u32, field: &str, _context: &bool, _extra_arg: &Option<String>) -> ParseResult<u32> {
	(
		1,
		Some(validation_error!(
			field.to_string(),
			"custom_code_2",
			"custom message 2"
		)),
	)
}

fn parse_three(_value: u32, field: &str, _context: &bool, _a: &Option<u32>, _b: &Option<u32>) -> ParseResult<u32> {
	(
		1,
		Some(validation_error!(
			field.to_string(),
			"custom_code_2",
			"custom message 2"
		)),
	)
}

#[test]
fn should_parse_customs_with_context() {
	let cases = [("4", false)];

	let mut wrapper = TestWrapper::default();
	for (case, context) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse_with_context(wrapper.clone(), context);

		assert_errors!(result, wrapper, {
			"a" => ("custom_code", "custom message"),
		});
	}
}
