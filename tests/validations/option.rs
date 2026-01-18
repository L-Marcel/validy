use serde::Deserialize;
use validation::{
	core::{Validate, ValidateAndParse},
	validation_errors,
};

use crate::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Deserialize, Validate, PartialEq)]
#[validate(payload)]
struct Test {
	#[validate(required("custom message"))]
	pub a: u8,
	pub b: Option<u8>,

	#[special(from_type(NestedTestWrapper))]
	#[validate(required(code = "custom_code"))]
	#[special(nested(NestedTest, NestedTestWrapper))]
	pub c: NestedTest,
}

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Default, Validate, PartialEq)]
#[validate(payload)]
struct NestedTest {
	#[validate(required("custom message", "custom_code"))]
	pub a: u8,
	pub b: Option<u8>,
}

#[test]
fn should_validate_and_parse_options() {
	let mut test = TestWrapper {
		a: None,
		b: Some(0),
		c: Some(NestedTestWrapper { a: None, b: Some(0) }),
	};

	let result = Test::validate_and_parse(&test);

	assert_errors!(result, test, {
		"a" => ("required", "custom message"),
		"c" => ("nested", validation_errors! {
		  "a" => ("custom_code", "custom message")
		})
	});

	test.a = Some(0);
	let result = Test::validate_and_parse(&test);

	assert_errors!(result, test, {
		"c" => ("nested", validation_errors! {
		  "a" => ("custom_code", "custom message")
		})
	});

	test.c = None;
	let result = Test::validate_and_parse(&test);

	assert_errors!(result, test, {
		"c" => ("custom_code", "is required")
	});

	test.c = Some(NestedTestWrapper { a: Some(0), b: None });
	let result = Test::validate_and_parse(&test);

	assert_parsed!(
		result,
		test,
		Test {
			a: 0,
			b: Some(0),
			c: NestedTest { a: 0, b: None }
		}
	);
}
