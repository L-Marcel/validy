use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use validy::core::Validate;

use validy::assert_errors;

use crate::utils::field_data::create_field_data_with_temp_file;

#[derive(Debug, Validate)]
#[validate(failure_mode = FailFast)]
struct Test {
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message", "custom_code"))]
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message 2", "custom_code_2"))]
	pub a: FieldData<NamedTempFile>,
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message", "custom_code"))]
	#[validate(field_content_type(r"^(application/json|text/css|image/.*)$", "custom message 2", "custom_code_2"))]
	pub b: Option<FieldData<NamedTempFile>>,
}

#[test]
fn should_validate_field_content_types() {
	let cases = ["text/html"];

	let mut test = Test {
		a: create_field_data_with_temp_file(),
		b: Some(create_field_data_with_temp_file()),
	};

	for case in cases.iter() {
		test.a.metadata.content_type = Some(case.to_string());
		let result = test.validate();

		assert_errors!(result, test, {
			"a" => ("custom_code", "custom message"),
		});
	}
}
