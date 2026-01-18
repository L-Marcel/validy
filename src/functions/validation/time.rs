use crate::core::ValidationError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::borrow::Cow;

pub fn validate_naive_date(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if NaiveDate::parse_from_str(value, format).is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_naive_time(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if NaiveDateTime::parse_from_str(value, format).is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_time(
	value: &str,
	format: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if DateTime::parse_from_str(value, format).is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_after<T: PartialOrd>(
	target: &T,
	reference: &T,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if (accept_equals && target < reference) || (!accept_equals && target <= reference) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_after_now<Tz: TimeZone>(
	target: &DateTime<Tz>,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let now = Utc::now().with_timezone(&target.timezone());

	let is_valid = if accept_equals {
		target.timestamp() >= now.timestamp()
	} else {
		*target > now
	};

	if !is_valid {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_before<T: PartialOrd>(
	target: &T,
	reference: &T,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	if (accept_equals && target < reference) || (!accept_equals && target >= reference) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_before_now<Tz: TimeZone>(
	target: &DateTime<Tz>,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let now = Utc::now().with_timezone(&target.timezone());

	let is_valid = if accept_equals {
		target.timestamp() <= now.timestamp()
	} else {
		*target < now
	};

	if !is_valid {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_now<Tz: TimeZone>(
	target: &DateTime<Tz>,
	ms_tolerance: i64,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let now = Utc::now().with_timezone(&target.timezone());
	let diff = now.signed_duration_since(target).num_milliseconds().abs();

	if diff > ms_tolerance {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_after_today(
	target: &NaiveDate,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let today = Utc::now().date_naive();

	if (accept_equals && *target < today) || (!accept_equals && *target <= today) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_before_today(
	target: &NaiveDate,
	accept_equals: bool,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let today = Utc::now().date_naive();

	if (accept_equals && *target > today) || (!accept_equals && *target >= today) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_is_today(
	target: &NaiveDate,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let today = Utc::now().date_naive();

	if *target != today {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
