use crate::core::ValidationError;
use std::borrow::Cow;

pub fn validate_none<U>(
	value: &U,
	items: impl IntoIterator<Item = U>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	U: PartialEq,
{
	if items.into_iter().any(|e| &e == value) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_any<U>(
	value: &U,
	items: impl IntoIterator<Item = U>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	U: PartialEq,
{
	if !items.into_iter().any(|e| &e == value) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
