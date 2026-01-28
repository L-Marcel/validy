use crate::fields::FieldAttributes;
use proc_macro2::TokenStream;
use quote::quote;

pub struct DefaultsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> DefaultsCodeFactory<'a> {
	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0
			.iter_mut()
			.map(|field| {
				let operations = field.get_operations();

				if field.is_option() {
					let original_reference = field.get_original_reference();
					let unwrapped = field.get_unwrapped_reference();

					quote! {
						if let Some(#unwrapped) = #original_reference.as_ref() {
							#(#operations)*
						}
					}
				} else {
					quote! {
					  #(#operations)*
					}
				}
			})
			.collect()
	}
}
