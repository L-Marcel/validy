use std::{cell::RefCell, collections::HashMap};

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		asynchronous::AsyncValidationFactory, asynchronous_modification::AsyncModificationFactory,
		asynchronous_modification_with_context::AsyncModificationWithContextFactory,
		asynchronous_payload::AsyncPayloadFactory, asynchronous_payload_with_context::AsyncPayloadWithContextFactory,
		asynchronous_with_context::AsyncValidationWithContextFactory, default::ValidationFactory,
		modification::ModificationFactory, modification_with_context::ModificationWithContextFactory,
		payload::PayloadFactory, payload_with_context::PayloadWithContextFactory,
		with_context::ValidationWithContextFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use syn::{Attribute, Ident, parse::ParseStream};

pub trait AbstractValidationFactory {
	fn create(
		&self,
		fields: Vec<FieldAttributes>,
		attributes: &ValidationAttributes,
		imports: &RefCell<ImportsSet>,
		struct_attributes: Vec<(Attribute, Import)>,
		fields_attributes: HashMap<String, Vec<(Attribute, Import)>>,
	) -> Output;
	fn create_nested(&self, input: ParseStream, field: &mut FieldAttributes) -> TokenStream;
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
		(Some(context), true, true, false) => Box::new(AsyncModificationWithContextFactory::new(name, context)),
		(Some(context), false, true, false) => Box::new(ModificationWithContextFactory::new(name, context)),
		(Some(context), true, _, true) => Box::new(AsyncPayloadWithContextFactory::new(name, context)),
		(Some(context), false, _, true) => Box::new(PayloadWithContextFactory::new(name, context)),
		(None, true, _, true) => Box::new(AsyncPayloadFactory::new(name)),
		(None, false, _, true) => Box::new(PayloadFactory::new(name)),
		(None, true, true, false) => Box::new(AsyncModificationFactory::new(name)),
		(None, false, true, false) => Box::new(ModificationFactory::new(name)),
		(None, true, false, false) => Box::new(AsyncValidationFactory::new(name)),
		_ => Box::new(ValidationFactory::new(name)),
	}
}
