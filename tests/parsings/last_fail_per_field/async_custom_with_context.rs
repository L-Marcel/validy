use validy::core::{AsyncValidateAndParseWithContext, ParseResult, Validate};
use validy::{assert_errors, validation_error};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, asynchronous, context = bool, failure_mode = LastFailPerField)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(async_custom_with_context(parse))]
	#[parse(async_custom_with_context(parse_two, [&wrapper.b]))]
	pub a: u32,
	#[special(from_type(String))]
	#[parse(async_custom_with_context(parse))]
	#[parse(async_custom_with_context(parse_three, [&tmp_3_a, &tmp_3_b]))]
	pub b: Option<u32>,
}

async fn parse(_value: String, field: &str, _context: &bool) -> ParseResult<u32> {
	(
		0,
		Some(validation_error!(field.to_string(), "custom_code", "custom message")),
	)
}

async fn parse_two(_value: u32, field: &str, _context: &bool, _extra_arg: &Option<String>) -> ParseResult<u32> {
	(
		1,
		Some(validation_error!(
			field.to_string(),
			"custom_code_2",
			"custom message 2"
		)),
	)
}

async fn parse_three(
	_value: u32,
	field: &str,
	_context: &bool,
	_a: &Option<u32>,
	_b: &Option<u32>,
) -> ParseResult<u32> {
	(
		1,
		Some(validation_error!(
			field.to_string(),
			"custom_code_2",
			"custom message 2"
		)),
	)
}

#[tokio::test]
async fn should_parse_async_customs_with_context() {
	let cases = [("4", false)];

	let mut wrapper = TestWrapper::default();
	for (case, context) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::async_validate_and_parse_with_context(wrapper.clone(), context).await;

		assert_errors!(result, wrapper, {
			"a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
