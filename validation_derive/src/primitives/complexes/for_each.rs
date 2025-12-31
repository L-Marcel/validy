use proc_macro2::TokenStream;
use quote::quote;
use syn::meta::ParseNestedMeta;

use crate::{
	attributes::ValidationAttributes,
	core::{get_complex_by_attr_macro, get_operation_by_attr_macro, get_validation_by_attr_macro},
	factories::core::AbstractValidationFactory,
	fields::FieldAttributes,
};

pub fn create_for_each(
	factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	let mut operations = Vec::<TokenStream>::new();
	let reference = field.get_reference();
	field.enter_scope();
	let item_reference = field.get_reference();

	let _ = meta.parse_nested_meta(|meta| {
		if meta.path.is_ident("validate") {
			let _ = meta.parse_nested_meta(|meta| {
				let validation = get_validation_by_attr_macro(factory, meta, field, attributes);
				operations.push(validation.clone());
				Ok(())
			});
		} else if meta.path.is_ident("modify") {
			let _ = meta.parse_nested_meta(|meta| {
				let operation = get_operation_by_attr_macro(factory, meta, field, attributes);
				operations.push(operation.clone());
				Ok(())
			});
		} else if meta.path.is_ident("complex") {
			let _ = meta.parse_nested_meta(|meta| {
				let operation = get_complex_by_attr_macro(factory, meta, field, attributes);
				operations.push(operation.clone());
				Ok(())
			});
		}
		Ok(())
	});

	field.exit_scope();

	quote! {
	  for #item_reference in #reference.into_iter() {
			#(#operations)*
	  }
	}
}
