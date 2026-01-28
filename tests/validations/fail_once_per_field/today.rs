use chrono::{Duration, NaiveDate, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
struct Test {
	#[validate(today("custom message", "custom_code"))]
	#[validate(today("custom message 2", "custom_code_2"))]
	pub a: NaiveDate,
	#[validate(today("custom message", "custom_code"))]
	#[validate(today("custom message 2", "custom_code_2"))]
	pub b: Option<NaiveDate>,
}

#[test]
fn should_validate_today() {
	let today = Utc::now().date_naive();

	let past = today - Duration::days(1);

	let cases = [past];

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
