use ::validy::core::ValidationError;
use std::borrow::Cow;
use uuid::Uuid;

pub fn validate_uuid(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if Uuid::parse_str(value).is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
