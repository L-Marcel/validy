use crate::core::ValidationError;
use std::{
	borrow::Cow,
	net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

pub fn validate_ip(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let ip: Result<IpAddr, _> = value.parse();

	if ip.is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

pub fn validate_ipv4(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let ip: Result<Ipv4Addr, _> = value.parse();

	if ip.is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	};

	Ok(())
}

pub fn validate_ipv6(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	let ip: Result<Ipv6Addr, _> = value.parse();

	if ip.is_err() {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}
