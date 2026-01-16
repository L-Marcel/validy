use chrono::{DateTime, Duration, FixedOffset, Utc};
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(after_now(true))]
	#[validate(after_now(true))]
	pub a: DateTime<FixedOffset>,
	#[validate(after_now(true, "custom message"))]
	#[validate(after_now(true, "custom message"))]
	pub b: Option<DateTime<FixedOffset>>,
	#[validate(after_now(true, code = "custom_code"))]
	pub c: Option<DateTime<FixedOffset>>,
	#[validate(after_now(true, "custom message", "custom_code"))]
	pub d: Option<DateTime<FixedOffset>>,
	#[validate(after_now(false))]
	#[validate(after_now(false))]
	pub e: DateTime<FixedOffset>,
}

#[test]
fn should_validate_after_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let future = now + Duration::days(1);
	let past = now - Duration::days(1);

	let cases = (
		[(past, false), (now, true), (future, true)],
		[(past, false), (now, false), (future, true)],
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
				"a" => ("after_now", "is before now"),
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
				"b" => ("after_now", "custom message"),
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
				"c" => ("custom_code", "is before now"),
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
			  "e" => ("after_now", "is before now"),
			});
		}
	}
}
