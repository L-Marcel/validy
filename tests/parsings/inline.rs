use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(inline(|x: String| x.parse::<u32>().unwrap_or(0)))]
	pub a: u32,
	#[special(from_type(String))]
	#[parse(inline(|x: String, a: &Option<u32>| a.unwrap_or(x.parse::<u32>().unwrap_or(0)), [&tmp_2_a]))]
	pub b: Option<u32>,
}

#[test]
fn should_parse_inlines() {
	let cases = [("4", 4), ("a", 0), ("8", 8)];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(result, wrapper, Test { a: *expected, b: None });
	}

	let last_a = result.expect("should be a valid result").a;
	for (case, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a,
				b: Some(last_a)
			}
		);
	}
}
