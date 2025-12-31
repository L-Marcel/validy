use crate::core::ValidationError;
use std::borrow::Cow;

pub fn validate_email(
	value: &str,
	field: impl Into<Cow<'static, str>>,
	code: impl Into<Cow<'static, str>>,
	message: impl Into<Cow<'static, str>>,
) -> Result<(), ValidationError> {
	use email_address::EmailAddress;
	if !EmailAddress::is_valid(value) {
		return Err(ValidationError::builder()
			.with_field(field)
			.as_simple(code)
			.with_message(message)
			.build()
			.into());
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_pass_valid_emails() {
		let emails = [
			("teste@gmail.com", true),
			("teste-hifen@gmail.com", true),
			("teste_sub@gmail.com", true),
			("teste@dominio-hifen.com", true),
			("teste..teste@gmail.com", false),
			("teste@gmail..com", false),
			(".teste@gmail.com", false),
			("teste.@gmail.com", false),
			("teste@.gmail.com", false),
			("teste@gmail.com.", false),
			("teste@-gmail.com", false),
			("teste@gmail-.com", false),
		];

		for (email, is_valid) in emails.iter() {
			let result = validate_email(email, "", "", "");

			if *is_valid {
				assert!(result.is_ok(), "{} {:?} for {}", is_valid, result, email);
			} else {
				assert!(result.is_err(), "{} {:?} for {}", is_valid, result, email);
			}
		}
	}
}
