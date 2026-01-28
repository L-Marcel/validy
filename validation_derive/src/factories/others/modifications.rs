use crate::fields::FieldAttributes;
use proc_macro2::TokenStream;
use quote::quote;

pub struct ModificationsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> ModificationsCodeFactory<'a> {
	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0
			.iter_mut()
			.map(|field| {
				if field.is_option() {
					let operations = &field.get_operations();
					let unwrapped = field.get_unwrapped_reference();
					let original_reference = field.get_original_reference();

					quote! {
						if let Some(#unwrapped) = #original_reference.as_mut() {
							#(#operations)*
						}
					}
				} else {
					let operations = &field.get_operations();
					quote! {
					  #(#operations)*
					}
				}
			})
			.collect()
	}
}
