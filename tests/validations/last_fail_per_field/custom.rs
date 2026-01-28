use serde::Deserialize;
use validy::{
	core::{Validate, ValidationError},
	validation_error,
};

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(custom(custom_fn, []))]
	#[validate(custom(custom_params_fn, [&self.b]))]
	pub a: bool,
	#[validate(custom(custom_fn, []))]
	#[validate(custom(custom_params_two_fn, [&self.a]))]
	pub b: Option<bool>,
}

pub fn custom_fn(_value: &bool, field: &str) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

pub fn custom_params_fn(_value: &bool, field: &str, _extra_param: &Option<bool>) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

pub fn custom_params_two_fn(_value: &bool, field: &str, _extra_param: &bool) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

#[test]
fn should_validate_customs() {
	let cases = [false];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
