use serde::Deserialize;
use validation::{
	core::{AsyncValidateWithContext, Validate, ValidationError},
	validation_error,
};

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(asynchronous, context = bool)]
struct Test {
	#[validate(async_custom_with_context(custom_fn, []))]
	#[validate(async_custom_with_context(custom_params_fn, [&self.b]))]
	pub a: bool,
	#[validate(async_custom_with_context(custom_fn, []))]
	#[validate(async_custom_with_context(custom_params_two_fn, [&self.a]))]
	pub b: Option<bool>,
	#[validate(async_custom_with_context(custom_fn, []))]
	pub c: Option<bool>,
	#[validate(async_custom_with_context(custom_fn, []))]
	pub d: Option<bool>,
}

pub async fn custom_fn(value: &bool, field: &str, context: &bool) -> Result<(), ValidationError> {
	if !*value || !*context {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

pub async fn custom_params_fn(
	value: &bool,
	field: &str,
	context: &bool,
	extra_param: &Option<bool>,
) -> Result<(), ValidationError> {
	if !(*value || extra_param.is_some_and(|c| c)) || !*context {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

pub async fn custom_params_two_fn(
	value: &bool,
	field: &str,
	context: &bool,
	extra_param: &bool,
) -> Result<(), ValidationError> {
	if !(*value && *extra_param && *context) {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

#[tokio::test]
async fn should_validate_async_customs_with_context() {
	let cases = [
		(false, false, false),
		(false, true, false),
		(true, false, false),
		(true, true, true),
	];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for (case, context, is_valid) in cases.iter() {
		test.a = *case;
		let result = test.async_validate_with_context(context).await;

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("custom_code", "custom message"),
				"b" => ("custom_code", "custom message"),
			});
		}
	}

	for (case, context, is_valid) in cases.iter() {
		test.b = Some(*case);
		let result = test.async_validate_with_context(context).await;

		if *is_valid {
			assert_validation!(result, test);
		} else if !*context {
			assert_errors!(result, test, {
			  "a" => ("custom_code", "custom message"),
				"b" => ("custom_code", "custom message"),
			});
		} else {
			assert_errors!(result, test, {
				"b" => ("custom_code", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, context, is_valid) in cases.iter() {
		test.c = Some(*case);
		let result = test.async_validate_with_context(context).await;

		if *is_valid {
			assert_validation!(result, test);
		} else if !*context {
			assert_errors!(result, test, {
			  "a" => ("custom_code", "custom message"),
				"c" => ("custom_code", "custom message"),
			});
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "custom message"),
			});
		}
	}

	test.c = None;
	for (case, context, is_valid) in cases.iter() {
		test.d = Some(*case);
		let result = test.async_validate_with_context(context).await;

		if *is_valid {
			assert_validation!(result, test);
		} else if !*context {
			assert_errors!(result, test, {
			  "a" => ("custom_code", "custom message"),
				"d" => ("custom_code", "custom message"),
			});
		} else {
			assert_errors!(result, test, {
				"d" => ("custom_code", "custom message"),
			});
		}
	}
}
