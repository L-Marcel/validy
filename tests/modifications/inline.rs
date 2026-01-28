use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Clone)]
struct Test {
	#[modificate(inline(|x: &mut String| *x = (x.to_string() + "_test").to_string()))]
	pub a: String,
	#[modificate(inline(|x: &mut String, a: &Option<String>| *x = a.clone().unwrap_or(x.to_string()), [&tmp_1_a]))]
	pub b: Option<String>,
}

#[test]
fn should_modificate_inlines() {
	let cases = [("a", "a_test"), ("b", "b_test"), ("c", "c_test")];

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
				a: expected.to_string(),
				b: None
			}
		);
	}

	let last_a = result.expect("should be a valid result").a;
	for (case, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone())
			}
		);
	}
}
