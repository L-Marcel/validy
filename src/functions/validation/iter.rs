use crate::core::ValidationError;
use std::borrow::Cow;

pub fn validate_blocklist<V, I, U, R>(
	values: V,
	items: I,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	U: PartialEq,
	R: Into<U>,
	V: IntoIterator<Item = U> + Clone,
	I: IntoIterator<Item = R> + Clone,
{
	let values: Vec<U> = values.clone().into_iter().collect();
	let items: Vec<U> = items.clone().into_iter().map(|item| item.into()).collect();

	if !values.iter().all(|value| !items.contains(value)) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_allowlist<V, I, U, R>(
	values: V,
	items: I,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	U: PartialEq,
	R: Into<U>,
	V: IntoIterator<Item = U> + Clone,
	I: IntoIterator<Item = R> + Clone,
{
	let values: Vec<U> = values.clone().into_iter().collect();
	let items: Vec<U> = items.clone().into_iter().map(|item| item.into()).collect();

	if !values.iter().all(|value| items.contains(value)) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
