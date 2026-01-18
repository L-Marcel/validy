use serde::Deserialize;
use validation::core::{Validate, ValidateAndModificate};

use crate::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modify)]
struct Test {
	#[modify(lower_camel_case)]
	#[modify(lower_camel_case)]
	pub a: String,
	#[modify(lower_camel_case)]
	pub b: Option<String>,
}

#[test]
fn should_modify_lower_camel_cases() {
	let cases = [
		("", ""),
		("hello", "hello"),
		("hello world", "helloWorld"),
		("hello_world", "helloWorld"),
		("hello-world", "helloWorld"),
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
