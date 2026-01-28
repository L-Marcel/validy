use chrono::{DateTime, Duration, FixedOffset, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(before_now(true, "custom message", "custom_code"))]
	#[validate(before_now(true, "custom message 2", "custom_code_2"))]
	pub a: DateTime<FixedOffset>,
	#[validate(before_now(true, "custom message", "custom_code"))]
	#[validate(before_now(true, "custom message 2", "custom_code_2"))]
	pub b: Option<DateTime<FixedOffset>>,
}

#[test]
fn should_validate_before_now() {
	let offset = FixedOffset::east_opt(0).unwrap();
	let now = Utc::now().with_timezone(&offset);

	let future = now + Duration::days(1);

	let cases = [future];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code", "custom message"),
		  "b" => ("custom_code", "custom message"),
		});
	}
}
