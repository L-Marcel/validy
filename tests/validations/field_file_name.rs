use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::{assert_errors, assert_validation};

use crate::utils::field_data::create_field_data_with_temp_file;

#[allow(unused)]
#[derive(Debug, Validate)]
struct Test {
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$"))]
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message"))]
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message"))]
	pub b: Option<FieldData<NamedTempFile>>,
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", code = "custom_code"))]
	pub c: Option<FieldData<NamedTempFile>>,
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message", "custom_code"))]
	pub d: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_file_names() {
	let cases = [
		("report_final.pdf", true),
		("image-01.jpg", true),
		("data.2023.csv", true),
		("UserProfile", true),
		("12345.png", true),
		("file name.txt", false),
		("folder/file.txt", false),
		("../secret.txt", false),
		("image*.jpg", false),
		("script.sh; rm -rf", false),
		("my_file?.png", false),
		(" file.txt", false),
		(".config", true),
		("", false),
		("archive_v2.tar.gz", true),
	];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: None,
		c: None,
		d: None,
	};

	for (case, is_valid) in cases.iter() {
		test.a.metadata.file_name = Some(case.to_string());
		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"a" => ("file_name", "invalid file name"),
			});
		}
	}

	test.b = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.b.as_mut() {
			file.metadata.file_name = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"b" => ("file_name", "custom message"),
			});
		}
	}

	test.b = None;
	test.c = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.c.as_mut() {
			file.metadata.file_name = Some(case.to_string());
		};

		let result = test.validate();

		if *is_valid {
			assert_validation!(result, test);
		} else {
			assert_errors!(result, test, {
				"c" => ("custom_code", "invalid file name"),
			});
		}
	}

	test.c = None;
	test.d = Some(create_field_data_with_temp_file());
	for (case, is_valid) in cases.iter() {
		if let Some(file) = test.d.as_mut() {
			file.metadata.file_name = Some(case.to_string());
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
