use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;

pub fn create_field_data_with_temp_file() -> FieldData<NamedTempFile> {
	let file = NamedTempFile::new().expect("should create a temporary file");

	FieldData {
		metadata: Default::default(),
		contents: file,
	}
}
