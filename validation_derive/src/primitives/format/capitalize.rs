use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_capitalize(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ModificationFunction("cases::capitalize as capitalize_fn"));

	let reference = field.get_reference();
	let field_name = field.get_name();

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) {
	      capitalize_fn(#reference);
			};
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
  		if can_continue(&errors, failure_mode, #field_name) {
        let _ref = &mut #reference;
        capitalize_fn(_ref);
  		};
		};

		result
	}
}
