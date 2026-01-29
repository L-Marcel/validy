use uuid::Uuid;
use validy::core::{Validate, ValidateAndParse};

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(uuid("custom message", "custom_code"))]
	pub a: Uuid,
	#[special(from_type(String))]
	#[parse(uuid("custom message", "custom_code"))]
	pub b: Option<Uuid>,
}

#[test]
fn should_parse_uuids() {
	let cases = [""];

	let mut wrapper = TestWrapper::default();
	for case in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse(wrapper.clone());

		assert_errors!(result, wrapper, {
			"a" => [("custom_code", "custom message")],
			"b" => [("custom_code", "custom message")],
		});
	}
}
