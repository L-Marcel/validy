use crate::{
	Output, ValidationAttributes,
	factories::{
		asynchronous::AsyncValidationFactory, asynchronous_with_context::AsyncValidationWithContextFactory,
		default::ValidationFactory, with_context::ValidationWithContextFactory,
	},
};
use proc_macro2::TokenStream;
use syn::{Ident, Type, meta::ParseNestedMeta};

pub trait AbstractValidationFactory {
	fn create(&self, operations: Vec<TokenStream>) -> Output;
	fn meta_is_custom(&self, meta: &ParseNestedMeta<'_>) -> bool;
	fn create_custom(&self, field_name: &Option<Ident>, meta: ParseNestedMeta<'_>) -> TokenStream;
	fn create_nested(&self, field_name: &Option<Ident>, field_type: &Type) -> TokenStream;
}

pub fn get_factory<'a>(
	name: &'a Ident,
	attributes: &'a ValidationAttributes,
) -> Box<dyn AbstractValidationFactory + 'a> {
	match (
		&attributes.context,
		&attributes.asynchronous,
		&attributes.modify,
		&attributes.payload,
	) {
		(Some(context), true, false, false) => Box::new(AsyncValidationWithContextFactory::new(name, context)),
		(Some(context), false, false, false) => Box::new(ValidationWithContextFactory::new(name, context)),
		(None, true, false, false) => Box::new(AsyncValidationFactory::new(name)),
		_ => Box::new(ValidationFactory::new(name)),
	}
}
