use ::validy::core::ValidationError;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use std::borrow::Cow;

pub fn default_naive_time() -> NaiveDateTime {
	Utc::now().naive_utc()
}

pub fn parse_naive_time(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (NaiveDateTime, Option<ValidationError>) {
	let result = NaiveDateTime::parse_from_str(value, format);

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Utc::now().naive_utc(),
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

pub fn default_time() -> DateTime<FixedOffset> {
	Utc::now().into()
}

pub fn parse_time(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (DateTime<FixedOffset>, Option<ValidationError>) {
	let result = DateTime::parse_from_str(value, format);

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Utc::now().into(),
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

pub fn default_naive_date() -> NaiveDate {
	Utc::now().date_naive()
}

pub fn parse_naive_date(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (NaiveDate, Option<ValidationError>) {
	let result = NaiveDate::parse_from_str(value, format);

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Utc::now().date_naive(),
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
