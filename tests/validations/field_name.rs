use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

use crate::utils::field_data::create_field_data_with_temp_file;

#[allow(unused)]
#[derive(Debug, Validate)]
struct Test {
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$"))]
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message"))]
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message"))]
	pub b: Option<FieldData<NamedTempFile>>,
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", code = "custom_code"))]
	pub c: Option<FieldData<NamedTempFile>>,
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message", "custom_code"))]
	pub d: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_names() {
	let cases = [
		("user_id", true),
		("firstName", true),
		("content-type", true),
		("v1", true),
		("API_KEY", true),
		("12345", true),
		("_private", true),
		("user name", false),
		("user.name", false),
		("user[name]", false),
		("/etc/passwd", false),
		("user@email", false),
		("script<alert>", false),
		("", false),
		("   ", false),
		("data_2024-v2", true),
	];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: None,
		c: None,
		d: None,
	};

	for (case, is_valid) in cases.iter() {
		test.a.metadata.name = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("field_name", "invalid field name"),
			});
		}
	}

	test.b = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.b.as_mut() {
			file.metadata.name = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("field_name", "custom message"),
			});
		}
	}

	test.b = None;
	test.c = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.c.as_mut() {
			file.metadata.name = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "invalid field name"),
			});
		}
	}

	test.c = None;
	test.d = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.d.as_mut() {
			file.metadata.name = Some(case.to_string());
		};

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
