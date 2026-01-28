use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

use crate::utils::field_data::create_field_data_with_temp_file;

#[allow(unused)]
#[derive(Debug, Validate)]
struct Test {
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$"))]
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message"))]
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message"))]
	pub b: Option<FieldData<NamedTempFile>>,
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", code = "custom_code"))]
	pub c: Option<FieldData<NamedTempFile>>,
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message", "custom_code"))]
	pub d: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_content_types() {
	let cases = [
		("application/json", true),
		("text/css", true),
		("image/png", true),
		("image/jpeg", true),
		("image/webp", true),
		("text/html", false),
		("application/pdf", false),
		("application/javascript", false),
		("text/plain", false),
		("json", false),
		("application/", false),
		("/css", false),
		("image", false),
		("", false),
		("image/gif", true),
	];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: None,
		c: None,
		d: None,
	};

	for (case, is_valid) in cases.iter() {
		test.a.metadata.content_type = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("content_type", "unsupported content type"),
			});
		}
	}

	test.b = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.b.as_mut() {
			file.metadata.content_type = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("content_type", "custom message"),
			});
		}
	}

	test.b = None;
	test.c = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.c.as_mut() {
			file.metadata.content_type = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "unsupported content type"),
			});
		}
	}

	test.c = None;
	test.d = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.d.as_mut() {
			file.metadata.content_type = Some(case.to_string());
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
