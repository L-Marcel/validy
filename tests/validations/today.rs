use chrono::{Duration, NaiveDate, Utc};
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(today)]
	#[validate(today)]
	pub a: NaiveDate,
	#[validate(today("custom message"))]
	#[validate(today("custom message"))]
	pub b: Option<NaiveDate>,
	#[validate(today(code = "custom_code"))]
	pub c: Option<NaiveDate>,
	#[validate(today("custom message", "custom_code"))]
	pub d: Option<NaiveDate>,
}

#[test]
fn should_validate_today() {
	let today = Utc::now().date_naive();

	let future = today + Duration::days(1);
	let past = today - Duration::days(1);

	let cases = [(past, false), (future, false), (today, true)];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("today", "isn't today"),
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
				"b" => ("today", "custom message"),
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
				"c" => ("custom_code", "isn't today"),
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
