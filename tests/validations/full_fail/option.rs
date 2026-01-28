use serde::Deserialize;
use validy::{
	assert_errors,
	core::{Validate, ValidateAndParse},
	validation_errors,
};

#[derive(Debug, Clone, Deserialize, Validate, PartialEq)]
#[validate(payload, failure_mode = FullFail)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[validate(required("custom message", "custom_code"))]
	pub a: u8,
	#[validate(required("custom message 2", "custom_code_2"))]
	pub b: u8,

	#[special(from_type(NestedTestWrapper))]
	#[validate(required("custom message 3", "custom_code_3"))]
	#[special(nested(NestedTest, NestedTestWrapper, "custom_code_4"))]
	pub c: NestedTest,
}

#[derive(Debug, Clone, Deserialize, Default, Validate, PartialEq)]
#[validate(payload, failure_mode = FullFail)]
#[wrapper_derive(Debug, Clone, Copy)]
struct NestedTest {
	#[validate(required("custom message", "custom_code"))]
	pub a: u8,
	#[validate(required("custom message 2", "custom_code_2"))]
	pub b: u8,
}

#[test]
fn should_validate_and_parse_options() {
	let test = TestWrapper {
		a: None,
		b: None,
		c: Some(NestedTestWrapper { a: None, b: None }),
	};

	let result = Test::validate_and_parse(test.clone());
	assert_errors!(result, test, {
	  "a" => ("custom_code", "custom message"),
	  "b" => ("custom_code_2", "custom message 2"),
		"c" => ("custom_code_4", validation_errors! {
		  "a" => ("custom_code", "custom message"),
			"b" => ("custom_code_2", "custom message 2"),
		})
	});
}
