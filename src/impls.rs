#[cfg(feature = "axum")]
use crate::core::NoContext;
use ::validy::core::{
	IntoValidationError, NestedValidationError, SimpleValidationError, ValidationError, ValidationErrors,
};
#[cfg(feature = "axum")]
use axum::extract::FromRef;
use std::{borrow::Cow, collections::HashMap};

impl NestedValidationError {
	pub fn from(errors: ValidationErrors, field: impl Into<Cow<'static, str>>) -> Self {
		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn from_with_code(
		errors: ValidationErrors,
		field: impl Into<Cow<'static, str>>,
		code: impl Into<Cow<'static, str>>,
	) -> Self {
		NestedValidationError {
			field: field.into(),
			code: code.into(),
			errors,
		}
	}

	pub fn new(field: impl Into<Cow<'static, str>>) -> Self {
		let errors = HashMap::<Cow<'static, str>, Vec<ValidationError>>::new();

		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn new_with_code(field: impl Into<Cow<'static, str>>, code: impl Into<Cow<'static, str>>) -> Self {
		let errors = HashMap::<Cow<'static, str>, Vec<ValidationError>>::new();

		NestedValidationError {
			field: field.into(),
			code: code.into(),
			errors,
		}
	}

	pub fn put(&mut self, error: ValidationError) {
		let field = match &error {
			ValidationError::Node(error) => error.field.clone(),
			ValidationError::Leaf(error) => error.field.clone(),
		};

		self.errors.entry(field).or_default().push(error);
	}
}

impl SimpleValidationError {
	pub fn new(field: impl Into<Cow<'static, str>>, code: impl Into<Cow<'static, str>>) -> Self {
		SimpleValidationError {
			field: field.into(),
			code: code.into(),
			message: None,
		}
	}

	pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
		self.message = Some(message.into());
		self
	}
}

impl From<NestedValidationError> for ValidationError {
	fn from(value: NestedValidationError) -> Self {
		ValidationError::Node(value)
	}
}

impl From<SimpleValidationError> for ValidationError {
	fn from(value: SimpleValidationError) -> Self {
		ValidationError::Leaf(value)
	}
}

impl IntoValidationError for &str {
	fn into_error(self, field: Cow<'static, str>, code: Cow<'static, str>) -> ValidationError {
		ValidationError::Leaf(SimpleValidationError {
			field,
			code,
			message: Some(self.to_string().into()),
		})
	}
}

impl IntoValidationError for ValidationErrors {
	fn into_error(self, field: Cow<'static, str>, code: Cow<'static, str>) -> ValidationError {
		ValidationError::Node(NestedValidationError {
			field,
			code,
			errors: self,
		})
	}
}

#[cfg(feature = "axum")]
impl<S> FromRef<S> for NoContext {
	fn from_ref(_: &S) -> Self {
		NoContext
	}
}
