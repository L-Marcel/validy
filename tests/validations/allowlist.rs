use std::collections::{HashMap, HashSet, VecDeque};

use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(allowlist("SINGLE", ["a", "b"]))]
	#[validate(allowlist("SINGLE", ["a", "b"]))]
	pub a: String,
	#[validate(allowlist("SINGLE", ["a", "b"], "custom message"))]
	#[validate(allowlist("SINGLE", ["a", "b"], "custom message"))]
	pub b: Option<String>,
	#[validate(allowlist("SINGLE", ["a", "b"], code = "custom_code"))]
	pub c: Option<String>,
	#[validate(allowlist("SINGLE", ["a", "b"], "custom message", "custom_code"))]
	pub d: Option<String>,
	#[validate(allowlist("COLLECTION", ["a", "b"]))]
	#[validate(allowlist("COLLECTION", ["a", "b"]))]
	pub e: Vec<String>,
	#[validate(allowlist("COLLECTION", [("a".to_string(), "c".to_string()), ("b".to_string(), "d".to_string())], "custom message"))]
	#[validate(allowlist("COLLECTION", [("a".to_string(), "c".to_string()), ("b".to_string(), "d".to_string())], "custom message"))]
	pub f: Option<HashMap<String, String>>,
	#[validate(allowlist("COLLECTION", ["a", "b"], code = "custom_code"))]
	pub g: Option<HashSet<String>>,
	#[validate(allowlist("COLLECTION", ["a", "b"], "custom message", "custom_code"))]
	pub h: Option<VecDeque<String>>,
	#[validate(allowlist("SINGLE", [0, 4], "custom message", "custom_code"))]
	#[validate(allowlist("SINGLE", [0, 4], "custom message", "custom_code"))]
	pub i: u8,
	#[validate(allowlist("SINGLE", [0, 4], "custom message", "custom_code"))]
	#[validate(allowlist("SINGLE", [0, 4], "custom message", "custom_code"))]
	pub j: Option<u8>,
}

#[test]
fn should_validate_allowlists() {
	let cases = (
		[("b", true), ("c", false), ("d", false), ("a", true)],
		[
			(Vec::<String>::new(), true),
			(vec!["a".into()], true),
			(vec!["b".into(), "c".into()], false),
			(vec!["b".into(), "a".into()], true),
		],
		[
			(HashMap::<String, String>::new(), true),
			(HashMap::from([("a".into(), "c".into())]), true),
			(
				HashMap::from([("a".into(), "c".into()), ("a".into(), "a".into())]),
				false,
			),
			(
				HashMap::from([("a".into(), "c".into()), ("b".into(), "d".into())]),
				true,
			),
		],
		[
			(HashSet::new(), true),
			(HashSet::from(["a".into()]), true),
			(HashSet::from(["b".into(), "c".into()]), false),
			(HashSet::from(["b".into(), "a".into()]), true),
		],
		[
			(VecDeque::new(), true),
			(VecDeque::from(["a".into()]), true),
			(VecDeque::from(["b".into(), "c".into()]), false),
			(VecDeque::from(["b".into(), "a".into()]), true),
		],
		[(0, true), (5, false), (6, false), (4, true)],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("allowlist", "has item outside allowlist"),
			});
		}
	}

	for (case, is_valid) in cases.0.iter() {
		test.b = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("allowlist", "custom message"),
			});
		}
	}

	test.b = None;
	for (case, is_valid) in cases.0.iter() {
		test.c = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "has item outside allowlist"),
			});
		}
	}

	test.c = None;
	for (case, is_valid) in cases.0.iter() {
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

	test.d = None;
	for (case, is_valid) in cases.1.iter() {
		test.e = case.clone();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"e" => ("allowlist", "has item outside allowlist"),
			});
		}
	}

	for (case, is_valid) in cases.2.iter() {
		test.f = Some(case.clone());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"f" => ("allowlist", "custom message"),
			});
		}
	}

	test.f = None;
	for (case, is_valid) in cases.3.iter() {
		test.g = Some(case.clone());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"g" => ("custom_code", "has item outside allowlist"),
			});
		}
	}

	test.g = None;
	for (case, is_valid) in cases.4.iter() {
		test.h = Some(case.clone());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"h" => ("custom_code", "custom message"),
			});
		}
	}

	test.h = None;
	for (case, is_valid) in cases.5.iter() {
		test.i = *case;
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"i" => ("custom_code", "custom message"),
			});
		}
	}

	for (case, is_valid) in cases.5.iter() {
		test.j = Some(*case);
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"j" => ("custom_code", "custom message"),
			});
		}
	}
}
