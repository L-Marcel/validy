use crate::fields::FieldAttributes;
use proc_macro2::TokenStream;
use quote::quote;

pub struct ModificationsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> ModificationsCodeFactory<'a> {
	pub fn commit(&self) -> TokenStream {
		let commits: Vec<TokenStream> = self
			.0
			.iter()
			.clone()
			.filter(|field| field.get_modifications() > 0)
			.map(|field| {
				if field.get_ignore() {
					return quote! {};
				};

				let reference = field.get_reference();
				let original_reference = field.get_original_reference();
				quote! {
				  #original_reference = #reference;
				}
			})
			.collect();

		#[rustfmt::skip]
		let commit = quote! {
		  #(#commits)*
      Ok(())
		};

		commit
	}

	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0.iter_mut().map(|field| field.get_operations()).collect()
	}
}
