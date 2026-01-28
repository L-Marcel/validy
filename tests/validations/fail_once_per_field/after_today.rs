use chrono::{Duration, NaiveDate, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(after_today(true, "custom message", "custom_code"))]
	#[validate(after_today(true, "custom message 2", "custom_code_2"))]
	pub a: NaiveDate,
	#[validate(after_today(true, "custom message", "custom_code"))]
	#[validate(after_today(true, "custom message 2", "custom_code_2"))]
	pub b: Option<NaiveDate>,
}

#[test]
fn should_validate_after_today() {
	let today = Utc::now().date_naive();

	let future = today + Duration::days(1);
	let past = today - Duration::days(1);

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
			"a" => ("custom_code", "custom message"),
			"b" => ("custom_code", "custom message")
		});
	}
}
