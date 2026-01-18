use serde::Deserialize;
use validation::core::{Validate, ValidateAndModificate};

use crate::{assert_modification, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
#[validate(modify)]
struct Test {
	#[modify(uppercase)]
	#[modify(uppercase)]
	pub a: String,
	#[modify(uppercase)]
	pub b: Option<String>,
}

#[test]
fn should_modify_uppercases() {
	let cases = [
		("", ""),
		(" abc", " ABC"),
		("abc ", "ABC "),
		("  aBc  ", "  ABC  "),
		("  a  B  C  ", "  A  B  C  "),
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
