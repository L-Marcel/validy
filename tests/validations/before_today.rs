use chrono::{Duration, NaiveDate, Utc};
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(before_today(true))]
	#[validate(before_today(true))]
	pub a: NaiveDate,
	#[validate(before_today(true, "custom message"))]
	#[validate(before_today(true, "custom message"))]
	pub b: Option<NaiveDate>,
	#[validate(before_today(true, code = "custom_code"))]
	pub c: Option<NaiveDate>,
	#[validate(before_today(true, "custom message", "custom_code"))]
	pub d: Option<NaiveDate>,
	#[validate(before_today(false))]
	#[validate(before_today(false))]
	pub e: NaiveDate,
}

#[test]
fn should_validate_before_today() {
	let today = Utc::now().date_naive();

	let future = today + Duration::days(1);
	let past = today - Duration::days(1);

	let cases = (
		[(future, false), (today, true), (past, true)],
		[(future, false), (today, false), (past, true)],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("before_today", "is after today"),
			});
		}
	}

	for (case, is_valid) in cases.0.iter() {
		test.b = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("before_today", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.0.iter() {
		test.c = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "is after today"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.0.iter() {
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

	test.d = None;
	for (case, is_valid) in cases.1.iter() {
		test.e = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"e" => ("before_today", "is after today"),
			});
		}
	}
}
