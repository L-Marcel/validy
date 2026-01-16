use proc_macro2::TokenStream;
use quote::quote;

use crate::fields::FieldAttributes;

pub fn create_trim(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	field.set_is_ref(false);

	quote! {
	  let mut #new_reference = #reference.trim().to_string();
	}
}
