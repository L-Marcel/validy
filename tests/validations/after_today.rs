use chrono::{Duration, NaiveDate, Utc};
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(after_today(true))]
	#[validate(after_today(true))]
	pub a: NaiveDate,
	#[validate(after_today(true, "custom message"))]
	#[validate(after_today(true, "custom message"))]
	pub b: Option<NaiveDate>,
	#[validate(after_today(true, code = "custom_code"))]
	pub c: Option<NaiveDate>,
	#[validate(after_today(true, "custom message", "custom_code"))]
	pub d: Option<NaiveDate>,
	#[validate(after_today(false))]
	#[validate(after_today(false))]
	pub e: NaiveDate,
}

#[test]
fn should_validate_after_today() {
	let today = Utc::now().date_naive();

	let future = today + Duration::days(1);
	let past = today - Duration::days(1);

	let cases = (
		[(past, false), (today, true), (future, true)],
		[(past, false), (today, false), (future, true)],
	);

	let mut test = Test {
		a: future,
		e: future,
		..Test::default()
	};

	for (case, is_valid) in cases.0.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("after_today", "is before today"),
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
				"b" => ("after_today", "custom message"),
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
				"c" => ("custom_code", "is before today"),
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
			  "e" => ("after_today", "is before today"),
			});
		}
	}
}
