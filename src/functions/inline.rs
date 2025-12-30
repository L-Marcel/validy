use std::borrow::Cow;

use crate::core::ValidationError;

pub fn validate_inline<U, F>(
	value: &U,
	inline: F,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	F: Fn(&U) -> bool,
{
	if !inline(value) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
