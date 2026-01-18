use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(url)]
	#[validate(url)]
	pub a: String,
	#[validate(url("custom message"))]
	#[validate(url("custom message"))]
	pub b: Option<String>,
	#[validate(url(code = "custom_code"))]
	pub c: Option<String>,
	#[validate(url("custom message", "custom_code"))]
	pub d: Option<String>,
}

#[test]
fn should_validate_urls() {
	let cases = [
		("http://site-com-hifen.com", true),
		("www.teste.org", true),
		("google.com", true),
		("sub.dominio.net", true),
		("site.com/caminho/para/rota", true),
		("site.com?q=busca&filtros=ok", true),
		("site.com.br", true),
		("google", false),
		("http://", false),
		("site.c", false),
		("site.abcdefg", false),
		("http://.com", false),
		("site.123", false),
		("https://www.google.com", true),
	];

	let mut test = Test::default();
	for (case, is_valid) in cases.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("url", "invalid url format"),
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
				"b" => ("url", "custom message"),
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
				"c" => ("custom_code", "invalid url format"),
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
