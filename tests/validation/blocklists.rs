use std::collections::{HashMap, HashSet, VecDeque};

use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(blocklist("SINGLE", ["a", "b"]))]
	pub a: Option<String>,
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message"))]
	pub b: Option<String>,
	#[validate(blocklist("SINGLE", ["a", "b"], code = "custom_code"))]
	pub c: Option<String>,
	#[validate(blocklist("SINGLE", ["a", "b"], "custom message", "custom_code"))]
	pub d: Option<String>,
	#[validate(blocklist("COLLECTION", ["a", "b"]))]
	pub e: Option<Vec<String>>,
	#[validate(blocklist("COLLECTION", [("a".to_string(), "c".to_string()), ("b".to_string(), "d".to_string())], "custom message"))]
	pub f: Option<HashMap<String, String>>,
	#[validate(blocklist("COLLECTION", ["a", "b"], code = "custom_code"))]
	pub g: Option<HashSet<String>>,
	#[validate(blocklist("COLLECTION", ["a", "b"], "custom message", "custom_code"))]
	pub h: Option<VecDeque<String>>,
}

#[test]
fn should_validate_blocklistss() {
	let cases = (
		[("a", false), ("b", false), ("c", true), ("d", true)],
		[
			(Vec::<String>::new(), true),
			(vec!["a".into()], false),
			(vec!["d".into(), "c".into()], true),
			(vec!["b".into(), "a".into()], false),
		],
		[
			(HashMap::<String, String>::new(), true),
			(HashMap::from([("a".into(), "c".into())]), false),
			(
				HashMap::from([("a".into(), "d".into()), ("a".into(), "a".into())]),
				true,
			),
			(
				HashMap::from([("a".into(), "c".into()), ("b".into(), "d".into())]),
				false,
			),
		],
		[
			(HashSet::new(), true),
			(HashSet::from(["a".into()]), false),
			(HashSet::from(["d".into(), "c".into()]), true),
			(HashSet::from(["b".into(), "a".into()]), false),
		],
		[
			(VecDeque::new(), true),
			(VecDeque::from(["a".into()]), false),
			(VecDeque::from(["d".into(), "c".into()]), true),
			(VecDeque::from(["b".into(), "a".into()]), false),
		],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("blocklist", "has item inside blocklist"),
			});
		}
	}

	test.a = None;
	for (case, is_valid) in cases.0.iter() {
		test.b = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("blocklist", "custom message"),
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
				"c" => ("custom_code", "has item inside blocklist"),
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
		test.e = Some(case.clone());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"e" => ("blocklist", "has item inside blocklist"),
			});
		}
	}

	test.e = None;
	for (case, is_valid) in cases.2.iter() {
		test.f = Some(case.clone());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"f" => ("blocklist", "custom message"),
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
				"g" => ("custom_code", "has item inside blocklist"),
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
}
