use chrono::{DateTime, FixedOffset};
use validy::core::{Validate, ValidateAndParse};

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload, failure_mode = FailFast)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(time("%Y-%m-%d %H:%M:%S %z", "custom message", "custom_code"))]
	pub a: DateTime<FixedOffset>,
	#[special(from_type(String))]
	#[parse(time("%Y-%m-%d %H:%M:%S %z", "custom message", "custom_code"))]
	pub b: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_parse_times() {
	let cases = [""];

	let mut wrapper = TestWrapper::default();
	for case in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse(wrapper.clone());

		assert_errors!(result, wrapper, {
			"a" => ("custom_code", "custom message"),
		});
	}
}
