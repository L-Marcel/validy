use chrono::{DateTime, Duration, FixedOffset, Utc};
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(now(5000))]
	#[validate(now(5000))]
	pub a: DateTime<FixedOffset>,
	#[validate(now(5000, "custom message"))]
	#[validate(now(5000, "custom message"))]
	pub b: Option<DateTime<FixedOffset>>,
	#[validate(now(5000, code = "custom_code"))]
	pub c: Option<DateTime<FixedOffset>>,
	#[validate(now(5000, "custom message", "custom_code"))]
	pub d: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_validate_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let future = now + Duration::days(1);
	let past = now - Duration::days(1);

	let cases = [(past, false), (future, false), (now, true)];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("now", "isn't now"),
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
				"b" => ("now", "custom message"),
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
				"c" => ("custom_code", "isn't now"),
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
