use serde::Deserialize;
use validy::{
	core::{Validate, ValidationError},
	validation_error,
};

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(custom(custom_fn, []))]
	#[validate(custom(custom_params_fn, [&self.b]))]
	pub a: bool,
	#[validate(custom(custom_fn, []))]
	#[validate(custom(custom_params_two_fn, [&self.a]))]
	pub b: Option<bool>,
	#[validate(custom(custom_fn, []))]
	pub c: Option<bool>,
	#[validate(custom(custom_fn, []))]
	pub d: Option<bool>,
}

pub fn custom_fn(value: &bool, field: &str) -> Result<(), ValidationError> {
	if !*value {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

pub fn custom_params_fn(value: &bool, field: &str, extra_param: &Option<bool>) -> Result<(), ValidationError> {
	if !(*value || extra_param.is_some_and(|c| c)) {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

pub fn custom_params_two_fn(value: &bool, field: &str, extra_param: &bool) -> Result<(), ValidationError> {
	if !(*value && *extra_param) {
		return Err(validation_error!(field.to_string(), "custom_code", "custom message"));
	}

	Ok(())
}

#[test]
fn should_validate_customs() {
	let cases = [(false, false), (true, true)];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for (case, is_valid) in cases.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("custom_code", "custom message"),
				"b" => ("custom_code", "custom message"),
			});
		}
	}

	for (case, is_valid) in cases.iter() {
		test.b = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("custom_code", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.iter() {
		test.c = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "custom message"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.iter() {
		test.d = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"d" => ("custom_code", "custom message"),
			});
		}
	}
}
