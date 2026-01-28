use ::validy::core::ValidationError;
use std::borrow::Cow;
use uuid::Uuid;

pub fn default_uuid() -> Uuid {
	Uuid::default()
}

pub fn parse_uuid(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (Uuid, Option<ValidationError>) {
	let result = Uuid::parse_str(value);

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Uuid::default(),
			Some(
				ValidationError::builder()
					.with_field(field)
					.as_simple(code)
					.with_message(message)
					.build()
					.into(),
			),
		)
	}
}
