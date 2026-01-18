use serde::Deserialize;
use validation::core::{Validate, ValidateAndModificate};

use crate::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modify)]
struct Test {
	#[modify(shouty_snake_case)]
	#[modify(shouty_snake_case)]
	pub a: String,
	#[modify(shouty_snake_case)]
	pub b: Option<String>,
}

#[test]
fn should_modify_shouty_snake_cases() {
	let cases = [
		("", ""),
		("hello", "HELLO"),
		("hello world", "HELLO_WORLD"),
		("hello_world", "HELLO_WORLD"),
		("hello-world", "HELLO_WORLD"),
	];

	let mut test = Test::default();
	for (case, expected) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate_and_modificate();

		assert_validation!(result, test);
		assert_modification!(test.a, expected.to_string(), test);
	}

	for (case, expected) in cases.iter() {
		test.b = Some(case.to_string());
		let result = test.validate_and_modificate();

		assert_validation!(result, test);
		assert_modification!(test.b, Some(expected.to_string()), test);
	}
}
