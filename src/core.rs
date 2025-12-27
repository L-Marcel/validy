use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;
#[cfg(feature = "derive")]
pub use validation_derive::*;

pub type ValidationErrors = HashMap<String, ValidationError>;

#[derive(Debug, Serialize)]
pub struct ValidationError {
	#[serde(skip_serializing)]
	pub field: String,
	pub code: String,
	pub message: String,
}

impl ValidationError {
	pub fn new(field: String, code: String, message: String) -> Self {
		ValidationError { field, code, message }
	}
}

pub trait Validation {
	fn validate(&self) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidation {
	async fn async_validate(&self) -> Result<(), ValidationErrors>;
}

#[async_trait]
impl<T> AsyncValidation for T
where
	T: Validation + Send + Sync,
{
	async fn async_validate(&self) -> Result<(), ValidationErrors> {
		self.validate()
	}
}

pub trait ValidationWithContext<C> {
	fn validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidationWithContext<C> {
	async fn async_validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
impl<T, C> AsyncValidationWithContext<C> for T
where
	T: ValidationWithContext<C> + Send + Sync,
	C: Send + Sync,
{
	async fn async_validate_with_context(&self, context: &C) -> Result<(), ValidationErrors> {
		self.validate_with_context(context)
	}
}
