use uuid::Uuid;
use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(uuid)]
	pub a: Uuid,
	#[special(from_type(String))]
	#[parse(uuid)]
	pub b: Option<Uuid>,
}

#[test]
fn should_parse_uuids() {
	let cases = [
		(
			"f47ac10b-58cc-4372-a567-0e02b2c3d479",
			Uuid::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479").expect("should be a valid uuid"),
		),
		(
			"urn:uuid:6ba7b810-9dad-11d1-80b4-00c04fd430c8",
			Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").expect("should be a valid uuid"),
		),
		("00000000-0000-0000-0000-000000000000", Uuid::nil()),
	];

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
	for (case, expected) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(wrapper.clone());

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a,
				b: Some(*expected)
			}
		);
	}

	wrapper.a = Some("invalid-uuid".to_string());
	result = Test::validate_and_parse(wrapper.clone());
	assert_errors!(result, wrapper, {
		"a" => ("uuid", "invalid uuid format"),
	});
}
