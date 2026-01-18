use std::collections::{HashMap, HashSet, VecDeque};

use serde::Deserialize;
use validation::core::Validate;

use crate::{assert_errors, assert_validation};

#[allow(unused)]
#[derive(Debug, Default, Deserialize, Validate, PartialEq)]
struct Test {
	#[validate(length(0..5))]
	#[validate(length(0..5))]
	pub a: String,
	#[validate(length(0..5, "custom message"))]
	#[validate(length(0..5, "custom message"))]
	pub b: Option<String>,
	#[validate(length(0..5, code = "custom_code"))]
	pub c: Option<String>,
	#[validate(length(0..5, "custom message", "custom_code"))]
	pub d: Option<String>,
	#[validate(length(0..5))]
	#[validate(length(0..5))]
	pub e: Vec<String>,
	#[validate(length(0..5, "custom message"))]
	#[validate(length(0..5, "custom message"))]
	pub f: Option<HashMap<String, String>>,
	#[validate(length(0..5, code = "custom_code"))]
	pub g: Option<HashSet<String>>,
	#[validate(length(0..5, "custom message", "custom_code"))]
	pub h: Option<VecDeque<String>>,
}

#[test]
fn should_validate_lengths() {
	let cases = (
		[
			("a", true),
			("ab", true),
			("abc", true),
			("abcd", true),
			("abcde", false),
			("abcdef", false),
			("", true),
		],
		[
			(Vec::<String>::new(), true),
			(vec!["a".into()], true),
			(vec!["a".into(); 2], true),
			(vec!["a".into(); 3], true),
			(vec!["a".into(); 5], false),
			(vec!["a".into(); 6], false),
			(vec!["a".into(); 4], true),
		],
		[
			(HashMap::<String, String>::new(), true),
			(HashMap::from([("k".into(), "v".into())]), true),
			((0..2).map(|i| (i.to_string(), "v".into())).collect(), true),
			((0..3).map(|i| (i.to_string(), "v".into())).collect(), true),
			((0..4).map(|i| (i.to_string(), "v".into())).collect(), true),
			((0..5).map(|i| (i.to_string(), "v".into())).collect(), false),
			((0..6).map(|i| (i.to_string(), "v".into())).collect(), false),
		],
		[
			(HashSet::new(), true),
			(HashSet::from(["k".into()]), true),
			((0..2).map(|i| i.to_string()).collect(), true),
			((0..3).map(|i| i.to_string()).collect(), true),
			((0..4).map(|i| i.to_string()).collect(), true),
			((0..5).map(|i| i.to_string()).collect(), false),
			((0..6).map(|i| i.to_string()).collect(), false),
		],
		[
			(VecDeque::new(), true),
			(VecDeque::from(["k".into()]), true),
			((0..2).map(|i| i.to_string()).collect(), true),
			((0..3).map(|i| i.to_string()).collect(), true),
			((0..4).map(|i| i.to_string()).collect(), true),
			((0..5).map(|i| i.to_string()).collect(), false),
			((0..6).map(|i| i.to_string()).collect(), false),
		],
	);

	let mut test = Test::default();
	for (case, is_valid) in cases.0.iter() {
		test.a = case.to_string();
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("length", "length out of range"),
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
				"b" => ("length", "custom message"),
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
				"c" => ("custom_code", "length out of range"),
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
				"e" => ("length", "length out of range"),
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
				"f" => ("length", "custom message"),
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
				"g" => ("custom_code", "length out of range"),
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
