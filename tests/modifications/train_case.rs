use serde::Deserialize;
use validation::core::{Validate, ValidateAndModificate};

use crate::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modify)]
struct Test {
	#[modify(train_case)]
	#[modify(train_case)]
	pub a: String,
	#[modify(train_case)]
	pub b: Option<String>,
}

#[test]
fn should_modify_train_cases() {
	let cases = [
		("", ""),
		("hello", "Hello"),
		("hello world", "Hello-World"),
		("hello_world", "Hello-World"),
		("hello-world", "Hello-World"),
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
