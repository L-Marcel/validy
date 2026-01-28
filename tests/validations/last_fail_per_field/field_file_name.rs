use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::assert_errors;

use crate::utils::field_data::create_field_data_with_temp_file;

#[derive(Debug, Validate)]
#[validate(failure_mode = LastFailPerField)]
struct Test {
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message", "custom_code"))]
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message 2", "custom_code_2"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message", "custom_code"))]
	#[validate(field_file_name(r"^[a-zA-Z0-9._-]+$", "custom message 2", "custom_code_2"))]
	pub b: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_file_names() {
	let cases = ["image*.jpg"];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: Some(create_field_data_with_temp_file()),
	};

	for case in cases.iter() {
		test.a.metadata.file_name = Some(case.to_string());
		if let Some(file) = test.b.as_mut() {
			file.metadata.file_name = Some(case.to_string());
		};

		let result = test.validate();

		assert_errors!(result, test, {
		  "a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
