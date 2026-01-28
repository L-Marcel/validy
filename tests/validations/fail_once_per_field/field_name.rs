use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::assert_errors;

use crate::utils::field_data::create_field_data_with_temp_file;

#[derive(Debug, Validate)]
struct Test {
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message", "custom_code"))]
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message 2", "custom_code_2"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message", "custom_code"))]
	#[validate(field_name(r"^[a-zA-Z0-9_-]+$", "custom message 2", "custom_code_2"))]
	pub b: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_names() {
	let cases = ["user.name"];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: Some(create_field_data_with_temp_file()),
	};

	for case in cases.iter() {
		test.a.metadata.name = Some(case.to_string());
		if let Some(file) = test.b.as_mut() {
			file.metadata.name = Some(case.to_string());
		};

		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code", "custom message"),
			"b" => ("custom_code", "custom message"),
		});
	}
}
