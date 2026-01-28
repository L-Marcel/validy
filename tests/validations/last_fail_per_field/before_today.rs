use chrono::{Duration, NaiveDate, Utc};
use validy::core::Validate;

use validy::assert_errors;

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(before_today(true, "custom message", "custom_code"))]
	#[validate(before_today(true, "custom message 2", "custom_code_2"))]
	pub a: NaiveDate,
	#[validate(before_today(true, "custom message", "custom_code"))]
	#[validate(before_today(true, "custom message 2", "custom_code_2"))]
	pub b: Option<NaiveDate>,
}

#[test]
fn should_validate_before_today() {
	let today = Utc::now().date_naive();

	let future = today + Duration::days(1);

	let cases = [future];

	let mut test = Test::default();
	for case in cases.iter() {
		test.a = *case;
		test.b = Some(*case);
		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
