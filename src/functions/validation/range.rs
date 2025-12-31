use std::{borrow::Cow, ops::RangeBounds};

use crate::core::ValidationError;

pub fn validate_range<R, T, U>(
	len: &U,
	range: R,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError>
where
	R: RangeBounds<T>,
	T: PartialOrd<U>,
	U: ?Sized + PartialOrd<T>,
	T: Sized,
{
	if !range.contains(len) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
