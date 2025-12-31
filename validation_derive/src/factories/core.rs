use crate::{
	Output,
	attributes::ValidationAttributes,
	factories::{
		asynchronous::AsyncValidationFactory, asynchronous_with_context::AsyncValidationWithContextFactory,
		default::ValidationFactory, with_context::ValidationWithContextFactory,
	},
	fields::FieldAttributes,
};
use proc_macro2::TokenStream;
use syn::Ident;

pub trait AbstractValidationFactory {
	fn create(&self, operations: Vec<TokenStream>) -> Output;
	fn create_nested(&self, field: &FieldAttributes) -> TokenStream;
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
		(Some(context), true, _, _) => Box::new(AsyncValidationWithContextFactory::new(name, context)),
		(Some(context), false, _, _) => Box::new(ValidationWithContextFactory::new(name, context)),
		(None, true, _, _) => Box::new(AsyncValidationFactory::new(name)),
		_ => Box::new(ValidationFactory::new(name)),
	}
}
