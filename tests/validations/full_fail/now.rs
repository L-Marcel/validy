use chrono::{DateTime, Duration, FixedOffset, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(failure_mode = FullFail)]
struct Test {
	#[validate(now(5000, "custom message", "custom_code"))]
	#[validate(now(5000, "custom message 2", "custom_code_2"))]
	pub a: DateTime<FixedOffset>,
	#[validate(now(5000, "custom message", "custom_code"))]
	#[validate(now(5000, "custom message 2", "custom_code_2"))]
	pub b: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_validate_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let past = now - Duration::days(1);

	let cases = [past];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
			"b" => [("custom_code", "custom message"), ("custom_code_2", "custom message 2")],
		});
	}
}
