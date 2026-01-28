use std::borrow::Cow;

use axum_typed_multipart::FieldData;

use ::validy::core::ValidationError;

pub fn validate_field_content_type<T>(
	value: &FieldData<T>,
	regex: impl Into<Cow<'static, str>>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	use ::validy::utils::regex::RegexManager;
	match RegexManager::get_or_create(regex) {
		Err(_) => Err(ValidationError::builder()
			.with_field(field)
			.as_simple("bad-regex")
			.with_message("can't build regex by provided pattern")
			.build()
			.into()),
		Ok(matcher) => {
			if let Some(content_type) = value.metadata.content_type.as_ref()
				&& matcher.is_match(content_type)
			{
				Ok(())
			} else {
				Err(ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into())
			}
		}
	}
}

pub fn validate_field_name<T>(
	value: &FieldData<T>,
	regex: impl Into<Cow<'static, str>>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	use ::validy::utils::regex::RegexManager;
	match RegexManager::get_or_create(regex) {
		Err(_) => Err(ValidationError::builder()
			.with_field(field)
			.as_simple("bad-regex")
			.with_message("can't build regex by provided pattern")
			.build()
			.into()),
		Ok(matcher) => {
			if let Some(field_name) = value.metadata.name.as_ref()
				&& matcher.is_match(field_name)
			{
				Ok(())
			} else {
				Err(ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into())
			}
		}
	}
}

pub fn validate_field_file_name<T>(
	value: &FieldData<T>,
	regex: impl Into<Cow<'static, str>>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	use ::validy::utils::regex::RegexManager;
	match RegexManager::get_or_create(regex) {
		Err(_) => Err(ValidationError::builder()
			.with_field(field)
			.as_simple("bad-regex")
			.with_message("can't build regex by provided pattern")
			.build()
			.into()),
		Ok(matcher) => {
			if let Some(field_file_name) = value.metadata.file_name.as_ref()
				&& matcher.is_match(field_file_name)
			{
				Ok(())
			} else {
				Err(ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into())
			}
		}
	}
}
