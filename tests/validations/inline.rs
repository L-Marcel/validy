use serde::Deserialize;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(inline(|x| x, []))]
	#[validate(inline(|x: &bool, b: &Option<bool>| b.is_some_and(|c| c) || *x, [&self.b]))]
	pub a: bool,
	#[validate(inline(|x| x, [], "custom message"))]
	#[validate(inline(|x: &bool, a: &bool| *a && *x, [&self.a], "custom message"))]
	pub b: Option<bool>,
	#[validate(inline(|x| x, [], code = "custom_code"))]
	pub c: Option<bool>,
	#[validate(inline(|x| x, [], "custom message", "custom_code"))]
	pub d: Option<bool>,
}

#[test]
fn should_validate_inlines() {
	let cases = [(false, false), (true, true)];

	let mut test = Test {
		b: Some(true),
		..Test::default()
	};

	for (case, is_valid) in cases.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("inline", "invalid"),
				"b" => ("inline", "custom message"),
			});
		}
	}

	for (case, is_valid) in cases.iter() {
		test.b = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("inline", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.iter() {
		test.c = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "invalid"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.iter() {
		test.d = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"d" => ("custom_code", "custom message"),
			});
		}
	}
}
