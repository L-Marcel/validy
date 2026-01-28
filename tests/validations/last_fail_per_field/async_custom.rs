use serde::Deserialize;
use validy::{
	core::{AsyncValidate, Validate, ValidationError},
	validation_error,
};

use validy::assert_errors;

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(asynchronous, failure_mode = LastFailPerField)]
struct Test {
	#[validate(async_custom(custom_fn, []))]
	#[validate(async_custom(custom_params_fn, [&self.b]))]
	pub a: bool,
	#[validate(async_custom(custom_fn, []))]
	#[validate(async_custom(custom_params_two_fn, [&self.a]))]
	pub b: Option<bool>,
}

pub async fn custom_fn(_value: &bool, field: &str) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

pub async fn custom_params_fn(_value: &bool, field: &str, _extra_param: &Option<bool>) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

pub async fn custom_params_two_fn(_value: &bool, field: &str, _extra_param: &bool) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

#[tokio::test]
async fn should_validate_async_customs() {
	let cases = [false];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.async_validate().await;

		assert_errors!(result, test, {
			"a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
