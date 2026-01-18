use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(range(0..5))]
	#[validate(range(0..5))]
	pub a: u8,
	#[validate(range(0..5, "custom message"))]
	#[validate(range(0..5, "custom message"))]
	pub b: Option<i8>,
	#[validate(range(0.0..5.0, code = "custom_code"))]
	pub c: Option<f32>,
	#[validate(range(0.0..5.0, "custom message", "custom_code"))]
	pub d: Option<f64>,
}

#[test]
fn should_validate_ranges() {
	let cases = (
		[
			(1, true),
			(2, true),
			(3, true),
			(4, true),
			(5, false),
			(6, false),
			(0, true),
		],
		[
			(1, true),
			(2, true),
			(3, true),
			(4, true),
			(5, false),
			(6, false),
			(0, true),
		],
		[
			(1.0, true),
			(2.0, true),
			(3.0, true),
			(4.9, true),
			(5.5, false),
			(6.0, false),
			(0.0, true),
		],
		[
			(1.0, true),
			(2.0, true),
			(3.0, true),
			(4.99, true),
			(5.5, false),
			(6.0, false),
			(0.0, true),
		],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("range", "out of range"),
			});
		}
	}

	for (case, is_valid) in cases.1.iter() {
		test.b = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("range", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.2.iter() {
		test.c = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "out of range"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.3.iter() {
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
