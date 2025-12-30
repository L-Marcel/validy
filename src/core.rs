use async_trait::async_trait;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
#[cfg(feature = "derive")]
pub use validation_derive::*;

use crate::builders::ValidationErrorBuilder;

pub type ValidationErrors = HashMap<Cow<'static, str>, ValidationError>;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ValidationError {
	Node(NestedValidationError),
	Leaf(SimpleValidationError),
}

impl ValidationError {
	pub fn builder() -> ValidationErrorBuilder {
		ValidationErrorBuilder {}
	}
}

#[derive(Debug, Serialize)]
pub struct NestedValidationError {
	#[serde(skip_serializing)]
	pub field: Cow<'static, str>,
	pub code: Cow<'static, str>,
	pub errors: ValidationErrors,
}

impl NestedValidationError {
	pub fn from(errors: ValidationErrors, field: impl Into<Cow<'static, str>>) -> Self {
		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn new(field: impl Into<Cow<'static, str>>) -> Self {
		let errors = HashMap::<Cow<'static, str>, ValidationError>::new();

		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn put(&mut self, error: ValidationError) {
		match error {
			ValidationError::Node(error) => {
				self.errors.insert(error.field.clone(), error.into());
			}
			ValidationError::Leaf(error) => {
				self.errors.insert(error.field.clone(), error.into());
			}
		}
	}
}

#[derive(Debug, Serialize)]
pub struct SimpleValidationError {
	#[serde(skip_serializing)]
	pub field: Cow<'static, str>,
	pub code: Cow<'static, str>,
	pub message: Option<Cow<'static, str>>,
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

pub trait Validation {
	fn validate(&self) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidation: Send + Sync {
	async fn async_validate(&self) -> Result<(), ValidationErrors>;
}

pub trait ValidationWithContext<C> {
	fn validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidationWithContext<C>: Send + Sync {
	async fn async_validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
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
