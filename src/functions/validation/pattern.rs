use crate::{core::ValidationError, utils::regex::RegexManager};
use std::borrow::Cow;

pub fn validate_pattern(
	value: &str,
	regex: impl Into<Cow<'static, str>>,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	match RegexManager::get_or_create(regex) {
		Err(_) => Err(ValidationError::builder()
			.with_field(field)
			.as_simple("bad-regex")
			.with_message("can't build regex by provided pattern")
			.build()
			.into()),
		Ok(matcher) => {
			if !matcher.is_match(value) {
				return Err(ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into());
			}

			Ok(())
		}
	}
}
