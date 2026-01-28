mod attributes;
mod core;
mod factories;
mod fields;
mod imports;
mod primitives;
mod types;

use std::cell::RefCell;

use crate::{
	attributes::get_attributes,
	core::{get_fields, get_fields_attributes},
	factories::core::get_factory,
	imports::ImportsSet,
	types::{Input, Output},
};

use proc_macro_error::{emit_error, proc_macro_error};
use syn::{DeriveInput, spanned::Spanned};

#[proc_macro_error]
#[proc_macro_derive(
	Validate,
	attributes(validate, modificate, parse, special, wrapper_derive, wrapper_attribute)
)]
pub fn validation_macro(input: Input) -> Output {
	let ast = syn::parse(input).unwrap();
	impl_validation_macro(&ast)
}

fn impl_validation_macro(ast: &DeriveInput) -> Output {
	let fields = get_fields(ast);
	let mut attributes = get_attributes(ast);
	let imports = RefCell::new(ImportsSet::new());

	if attributes.modificate && attributes.payload {
		emit_error!(ast.span(), "payload implies modificate");
	}

	attributes.modificate = attributes.modificate || attributes.payload;

	let mut factory = get_factory(&ast.ident, &attributes);
	factory.init(ast, &attributes);

	let fields_attributes = get_fields_attributes(fields, factory.as_ref(), &attributes, &imports);

	factory.create(fields_attributes, &attributes, &imports)
}
