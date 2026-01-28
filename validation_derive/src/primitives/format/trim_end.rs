use proc_macro2::TokenStream;
use quote::quote;

use crate::fields::FieldAttributes;

pub fn create_trim_end(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	let field_name = field.get_name();

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
  		if can_continue(&errors, failure_mode, #field_name) {
  		  *#reference = #reference.trim_end().to_string();
  		};
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) {
			  let _ref = &mut #reference;
			  *_ref = _ref.trim_end().to_string();
  	  };
		};

		result
	}
}
