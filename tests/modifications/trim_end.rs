use serde::Deserialize;
use validy::core::{Validate, ValidateAndModificate};

use validy::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modificate)]
struct Test {
	#[modificate(trim_end)]
	#[modificate(trim_end)]
	pub a: String,
	#[modificate(trim_end)]
	pub b: Option<String>,
}

#[test]
fn should_modificate_trims_end() {
	let cases = [
		("", ""),
		(" abc", " abc"),
		("abc ", "abc"),
		("  abc  ", "  abc"),
		("  a  b  c  ", "  a  b  c"),
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
