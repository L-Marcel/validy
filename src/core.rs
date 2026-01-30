use ::validy::builders::ValidationErrorBuilder;
use async_trait::async_trait;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
#[cfg(feature = "derive")]
pub use validation_derive::*;

#[cfg(feature = "modification")]
pub type ParseResult<T> = (T, Option<ValidationError>);
pub type ValidationErrors = HashMap<Cow<'static, str>, Vec<ValidationError>>;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct NestedValidationError {
	#[serde(skip_serializing)]
	pub field: Cow<'static, str>,
	pub code: Cow<'static, str>,
	pub errors: ValidationErrors,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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

pub trait SpecificValidateWithContext {
	type Context: Send + Sync;
	fn specific_validate_with_context(&self, context: &Self::Context) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateWithContext<C>: Send + Sync {
	async fn async_validate_with_context(&self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait SpecificAsyncValidateWithContext: Send + Sync {
	type Context: Send + Sync;
	async fn specific_async_validate_with_context(&self, context: &Self::Context) -> Result<(), ValidationErrors>;
}

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

pub trait SpecificValidateAndModificateWithContext {
	type Context: Send + Sync;
	fn specific_validate_and_modificate_with_context(
		&mut self,
		context: &Self::Context,
	) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateAndModificateWithContext<C>: Send + Sync {
	async fn async_validate_and_modificate_with_context(&mut self, context: &C) -> Result<(), ValidationErrors>;
}

#[async_trait]
pub trait SpecificAsyncValidateAndModificateWithContext: Send + Sync {
	type Context: Send + Sync;
	async fn specific_async_validate_and_modificate_with_context(
		&mut self,
		context: &Self::Context,
	) -> Result<(), ValidationErrors>;
}

pub trait ValidateAndParse<W>: Sized {
	fn validate_and_parse(wrapper: W) -> Result<Self, ValidationErrors>;
}

pub trait SpecificValidateAndParse: Sized {
	type Wrapper: Send + Sync;
	fn specific_validate_and_parse(wrapper: Self::Wrapper) -> Result<Self, ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateAndParse<W>: Sized + Send + Sync {
	async fn async_validate_and_parse(wrapper: W) -> Result<Self, ValidationErrors>;
}

#[async_trait]
pub trait SpecificAsyncValidateAndParse: Sized + Send + Sync {
	type Wrapper: Send + Sync;
	async fn specific_async_validate_and_parse(wrapper: Self::Wrapper) -> Result<Self, ValidationErrors>;
}

pub trait ValidateAndParseWithContext<W, C>: Sized {
	fn validate_and_parse_with_context(wrapper: W, context: &C) -> Result<Self, ValidationErrors>;
}

pub trait SpecificValidateAndParseWithContext: Sized {
	type Wrapper: Send + Sync;
	type Context: Send + Sync;
	fn specific_validate_and_parse_with_context(
		wrapper: Self::Wrapper,
		context: &Self::Context,
	) -> Result<Self, ValidationErrors>;
}

#[async_trait]
pub trait AsyncValidateAndParseWithContext<W, C>: Sized + Send + Sync {
	async fn async_validate_and_parse_with_context(wrapper: W, context: &C) -> Result<Self, ValidationErrors>;
}

#[async_trait]
pub trait SpecificAsyncValidateAndParseWithContext: Sized + Send + Sync {
	type Wrapper: Send + Sync;
	type Context: Send + Sync;
	async fn specific_async_validate_and_parse_with_context(
		wrapper: Self::Wrapper,
		context: &Self::Context,
	) -> Result<Self, ValidationErrors>;
}

#[derive(Debug)]
pub struct NoContext;

pub trait IntoValidationError {
	fn into_error(self, field: Cow<'static, str>, code: Cow<'static, str>) -> ValidationError;
}
