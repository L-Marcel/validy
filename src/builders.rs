use std::borrow::Cow;

use crate::core::{NestedValidationError, SimpleValidationError, ValidationErrors};

pub struct ValidationErrorBuilder {}

impl ValidationErrorBuilder {
	pub fn with_field(self, field: impl Into<Cow<'static, str>>) -> ValidationErrorBuilderWithField {
		ValidationErrorBuilderWithField { field: field.into() }
	}
}

pub struct ValidationErrorBuilderWithField {
	pub(super) field: Cow<'static, str>,
}

impl ValidationErrorBuilderWithField {
	pub fn as_simple(self, code: impl Into<Cow<'static, str>>) -> SimpleValidationErrorBuilder {
		SimpleValidationErrorBuilder {
			code: code.into(),
			field: self.field,
			message: None,
		}
	}

	pub fn as_nested(self) -> NestedValidationErrorBuilder {
		NestedValidationErrorBuilder {
			field: self.field,
			errors: ValidationErrors::new(),
		}
	}
}

pub struct SimpleValidationErrorBuilder {
	pub(super) code: Cow<'static, str>,
	pub(super) field: Cow<'static, str>,
	pub(super) message: Option<Cow<'static, str>>,
}

impl SimpleValidationErrorBuilder {
	pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> SimpleValidationErrorBuilder {
		self.message = Some(message.into());
		self
	}

	pub fn build(self) -> SimpleValidationError {
		SimpleValidationError {
			code: self.code,
			field: self.field,
			message: self.message,
		}
	}
}

pub struct NestedValidationErrorBuilder {
	pub(super) field: Cow<'static, str>,
	pub(super) errors: ValidationErrors,
}

impl NestedValidationErrorBuilder {
	pub fn with_errors(mut self, errors: ValidationErrors) -> NestedValidationErrorBuilder {
		self.errors = errors;
		self
	}

	pub fn with_error(mut self, error: SimpleValidationError) -> NestedValidationErrorBuilder {
		self.errors.insert(error.field.clone(), error.into());
		self
	}

	pub fn without_error(mut self, field: impl Into<Cow<'static, str>>) -> NestedValidationErrorBuilder {
		self.errors.remove(&field.into());
		self
	}

	pub fn build(self) -> NestedValidationError {
		NestedValidationError::from(self.errors, self.field)
	}
}
