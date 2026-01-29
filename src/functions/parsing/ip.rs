use std::{
	borrow::Cow,
	net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use crate::core::ValidationError;

pub fn default_ip() -> IpAddr {
	IpAddr::V4(Ipv4Addr::UNSPECIFIED)
}

pub fn parse_ip(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (IpAddr, Option<ValidationError>) {
	let result: Result<IpAddr, _> = value.parse();

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			IpAddr::V4(Ipv4Addr::UNSPECIFIED),
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

pub fn default_ipv4() -> Ipv4Addr {
	Ipv4Addr::UNSPECIFIED
}

pub fn parse_ipv4(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (Ipv4Addr, Option<ValidationError>) {
	let result: Result<Ipv4Addr, _> = value.parse();

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Ipv4Addr::UNSPECIFIED,
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

pub fn default_ipv6() -> Ipv6Addr {
	Ipv6Addr::UNSPECIFIED
}

pub fn parse_ipv6(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> (Ipv6Addr, Option<ValidationError>) {
	let result: Result<Ipv6Addr, _> = value.parse();

	if let Ok(result) = result {
		(result, None)
	} else {
		(
			Ipv6Addr::UNSPECIFIED,
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
