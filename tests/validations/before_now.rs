use chrono::{DateTime, Duration, FixedOffset, Utc};
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(before_now(true))]
	#[validate(before_now(true))]
	pub a: DateTime<FixedOffset>,
	#[validate(before_now(true, "custom message"))]
	#[validate(before_now(true, "custom message"))]
	pub b: Option<DateTime<FixedOffset>>,
	#[validate(before_now(true, code = "custom_code"))]
	pub c: Option<DateTime<FixedOffset>>,
	#[validate(before_now(true, "custom message", "custom_code"))]
	pub d: Option<DateTime<FixedOffset>>,
	#[validate(before_now(false))]
	#[validate(before_now(false))]
	pub e: DateTime<FixedOffset>,
}

#[test]
fn should_validate_before_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let future = now + Duration::days(1);
	let past = now - Duration::days(1);

	let cases = (
		[(future, false), (now, true), (past, true)],
		[(future, false), (past, true)],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("before_now", "is after now"),
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
				"b" => ("before_now", "custom message"),
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
				"c" => ("custom_code", "is after now"),
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
				"e" => ("before_now", "is after now"),
			});
		}
	}
}
