use proc_macro2::TokenStream;
use quote::quote;

use crate::{fields::FieldAttributes, imports::import_modification_functions};

pub fn create_capitalize(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let import = import_modification_functions("capitalize::capitalize");

	quote! {
	  use #import;
		let mut #new_reference = capitalize(&#reference);
	}
}
