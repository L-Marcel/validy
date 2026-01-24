use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_shouty_kebab_case(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ModificationFunction(
		"cases::shouty_kebab_case as shouty_kebab_case_fn",
	));

	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let field_name = field.get_name();

	if field.is_ref() {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
			let mut #new_reference = if can_continue(&errors, failure_mode, #field_name) {
	      shouty_kebab_case_fn(#reference)
			} else {
			  #reference.clone()
			};
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
			let mut #new_reference = if can_continue(&errors, failure_mode, #field_name) {
  			shouty_kebab_case_fn(&#reference)
  	  } else {
  			#reference
  	  };
		};

		result
	}
}
