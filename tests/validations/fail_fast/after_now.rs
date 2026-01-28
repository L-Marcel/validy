use chrono::{DateTime, Duration, FixedOffset, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(failure_mode = FailFast)]
struct Test {
	#[validate(after_now(true, "custom message", "custom_code"))]
	#[validate(after_now(true, "custom message 2", "custom_code_2"))]
	pub a: DateTime<FixedOffset>,
	#[validate(after_now(true, "custom message", "custom_code"))]
	#[validate(after_now(true, "custom message 2", "custom_code_2"))]
	pub b: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_validate_after_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let future = now + Duration::days(1);
	let past = now - Duration::days(1);

	let cases = [past];

	let mut test = Test {
		a: future,
		..Test::default()
	};

	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code", "custom message")
		});
	}
}
