use serde::Deserialize;
use validy::core::{Validate, ValidateAndModificate};

use validy::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modificate)]
struct Test {
	#[modificate(shouty_kebab_case)]
	#[modificate(shouty_kebab_case)]
	pub a: String,
	#[modificate(shouty_kebab_case)]
	pub b: Option<String>,
}

#[test]
fn should_modificate_shouty_kebab_cases() {
	let cases = [
		("", ""),
		("hello", "HELLO"),
		("hello world", "HELLO-WORLD"),
		("hello_world", "HELLO-WORLD"),
		("hello-world", "HELLO-WORLD"),
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
