use validy::core::{AsyncValidateAndParse, Validate, ValidationError};
use validy::{assert_errors, assert_parsed, validation_error};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, asynchronous)]
#[wrapper_derive(Clone)]
struct Test {
	#[modificate(async_custom(modificate))]
	pub a: String,
	#[modificate(async_custom(modificate_two, [&tmp_1_a]))]
	pub b: Option<String>,
	#[modificate(async_custom(modificate_three, [&tmp_1_a, &tmp_1_b]))]
	pub c: Option<String>,
}

async fn modificate(value: &mut String, _field_name: &str) -> Result<(), ValidationError> {
	*value = (value.to_string() + "_test").to_string();
	Ok(())
}

async fn modificate_two(
	value: &mut String,
	_field_name: &str,
	extra_arg: &Option<String>,
) -> Result<(), ValidationError> {
	*value = extra_arg.clone().unwrap_or(value.to_string());
	Ok(())
}

async fn modificate_three(
	value: &mut String,
	field_name: &str,
	a: &Option<String>,
	b: &Option<String>,
) -> Result<(), ValidationError> {
	match (a, b) {
		(_, None) => Err(validation_error!(
			field_name.to_string(),
			"custom_code",
			"custom message"
		)),
		(Some(a), _) => {
			*value = a.to_string().clone();
			Ok(())
		}
		(None, Some(b)) => {
			*value = b.to_string().clone();
			Ok(())
		}
	}
}

#[tokio::test]
async fn should_modificate_customs() {
	let cases = [("a", "a_test"), ("b", "b_test"), ("c", "c_test")];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::async_validate_and_parse(wrapper.clone()).await;
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::async_validate_and_parse(wrapper.clone()).await;

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: expected.to_string(),
				b: None,
				c: None,
			}
		);
	}

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		result = Test::async_validate_and_parse(wrapper.clone()).await;

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: expected.to_string(),
				b: Some(expected.to_string()),
				c: None
			}
		);
	}

	wrapper.c = Some("".to_string());
	wrapper.b = None;
	result = Test::async_validate_and_parse(wrapper.clone()).await;
	assert_errors!(result, wrapper, {
	  "c" => ("custom_code", "custom message")
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		wrapper.c = Some(case.to_string());
		result = Test::async_validate_and_parse(wrapper.clone()).await;

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: expected.to_string(),
				b: Some(expected.to_string()),
				c: Some(expected.to_string())
			}
		);
	}
}
