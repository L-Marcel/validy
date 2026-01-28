use std::borrow::Cow;

use ::validy::core::ValidationError;

pub fn validate_blocklist<V, I>(
	values: V,
	items: I,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	V: IntoIterator,
	I: IntoIterator + Clone,
	V::Item: PartialEq<I::Item>,
{
	for val in values {
		for item in items.clone() {
			if val == item {
				return Err(ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into());
			}
		}
	}
	Ok(())
}

pub fn validate_allowlist<V, I>(
	values: V,
	items: I,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	V: IntoIterator,
	I: IntoIterator + Clone,
	V::Item: PartialEq<I::Item>,
{
	for val in values {
		let mut found = false;
		for item in items.clone() {
			if val == item {
				found = true;
				break;
			}
		}

		if !found {
			return Err(ValidationError::builder()
				.with_field(field)
				.as_simple(code)
				.with_message(message)
				.build()
				.into());
		}
	}
	Ok(())
}
