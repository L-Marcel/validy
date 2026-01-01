use crate::builders::ValidationErrorBuilder;
use async_trait::async_trait;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
#[cfg(feature = "derive")]
pub use validation_derive::*;

#[cfg(feature = "modification")]
pub type ModificationResult<T> = (T, Option<ValidationError>);
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

#[derive(Debug, Serialize)]
pub struct SimpleValidationError {
	#[serde(skip_serializing)]
	pub field: Cow<'static, str>,
	pub code: Cow<'static, str>,
	pub message: Option<Cow<'static, str>>,
}

pub trait Validate {
	fn validate(&self) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidate: Send + Sync {
	async fn async_validate(&self) -> Result<(), ValidationErrors>;
}

pub trait ValidateWithContext<C> {
	fn validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateWithContext<C>: Send + Sync {
	async fn async_validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

//...

pub trait ValidateAndModificate {
	fn validate_and_modificate(&mut self) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateAndModificate: Send + Sync {
	async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors>;
}

pub trait ValidateAndModificateWithContext<C> {
	fn validate_and_modificate_with_context(&mut self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateAndModificateWithContext<C>: Send + Sync {
	async fn async_validate_and_modificate_with_context(&mut self, context: &C) -> Result<(), ValidationErrors>;
}

//....

pub trait DeserializeAndValidate {
	fn deserialize_and_validate(&mut self) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncDeserializeAndValidate: Send + Sync {
	async fn async_deserialize_and_validate(&mut self) -> Result<(), ValidationErrors>;
}

pub trait DeserializeAndValidateWithContext<C> {
	fn deserialize_and_validatewith_context(&mut self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncDeserializeAndValidateWithContext<C>: Send + Sync {
	async fn async_deserialize_and_validate_with_context(&mut self, context: &C) -> Result<(), ValidationErrors>;
}
