use serde::Deserialize;
use validy::core::{Validate, ValidateAndModificate};

use validy::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modificate)]
struct Test {
	#[modificate(lower_camel_case)]
	#[modificate(lower_camel_case)]
	pub a: String,
	#[modificate(lower_camel_case)]
	pub b: Option<String>,
}

#[test]
fn should_modificate_lower_camel_cases() {
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
