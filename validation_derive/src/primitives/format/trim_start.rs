use proc_macro2::TokenStream;
use quote::quote;

use crate::fields::FieldAttributes;

pub fn create_trim_start(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let field_name = field.get_name();

	field.set_is_ref(false);

	#[rustfmt::skip]
	let result = quote! {
		let mut #new_reference = if can_continue(&errors, failure_mode, #field_name) {
		  #reference.trim_start().to_string()
		} else {
		  #reference.clone()
		};
	};

	result
}
