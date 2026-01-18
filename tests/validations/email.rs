use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(email)]
	#[validate(email)]
	pub a: String,
	#[validate(email("custom message"))]
	#[validate(email("custom message"))]
	pub b: Option<String>,
	#[validate(email(code = "custom_code"))]
	pub c: Option<String>,
	#[validate(email("custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_emails() {
	let cases = [
		("teste-hifen@gmail.com", true),
		("teste_sub@gmail.com", true),
		("teste@dominio-hifen.com", true),
		("teste..teste@gmail.com", false),
		("teste@gmail..com", false),
		(".teste@gmail.com", false),
		("teste.@gmail.com", false),
		("teste@.gmail.com", false),
		("teste@gmail.com.", false),
		("teste@-gmail.com", false),
		("teste@gmail-.com", false),
		("teste@gmail.com", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("email", "invalid email format"),
			});
		}
	}

	for (case, is_valid) in cases.iter() {
		test.b = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("email", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.iter() {
		test.c = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "invalid email format"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.iter() {
		test.d = Some(case.to_string());
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
