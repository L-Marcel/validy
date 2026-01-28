use serde::Deserialize;
use validy::{
	core::{Validate, ValidateWithContext, ValidationError},
	validation_error,
};

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(context = bool, failure_mode = FullFail)]
struct Test {
	#[validate(custom_with_context(custom_fn, []))]
	#[validate(custom_with_context(custom_params_fn, [&self.b]))]
	pub a: bool,
	#[validate(custom_with_context(custom_fn, []))]
	#[validate(custom_with_context(custom_params_two_fn, [&self.a]))]
	pub b: Option<bool>,
}

pub fn custom_fn(_value: &bool, field: &str, _context: &bool) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

pub fn custom_params_fn(
	_value: &bool,
	field: &str,
	_context: &bool,
	_extra_param: &Option<bool>,
) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

pub fn custom_params_two_fn(
	_value: &bool,
	field: &str,
	_context: &bool,
	_extra_param: &bool,
) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

#[test]
fn should_validate_customs_with_context() {
	let cases = [(false, false)];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for (case, context) in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate_with_context(context);

		assert_errors!(result, test, {
			"a" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
			"b" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
		});
	}
}
