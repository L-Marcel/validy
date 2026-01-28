use serde::Deserialize;
use validy::core::{Validate, ValidateAndModificate};

use validy::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modificate)]
struct Test {
	#[modificate(camel_case)]
	#[modificate(camel_case)]
	pub a: String,
	#[modificate(camel_case)]
	pub b: Option<String>,
}

#[test]
fn should_modificate_camel_cases() {
	let cases = [
		("", ""),
		("hello", "Hello"),
		("hello world", "HelloWorld"),
		("hello_world", "HelloWorld"),
		("hello-world", "HelloWorld"),
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
