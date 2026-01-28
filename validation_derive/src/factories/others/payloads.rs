use crate::fields::FieldAttributes;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub struct PayloadsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> PayloadsCodeFactory<'a> {
	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0
			.iter_mut()
			.map(|field| {
				let field_name = field.get_name();
				let wrapper_final_type = field.get_wrapper_final_type();
				let reference = field.get_reference();
				field.increment_modifications();
				let operations = field.get_operations();
				let new_reference = field.get_reference();
				let wrapper_reference = field.get_wrapper_reference();
				let unwrapped = field.get_unwrapped_reference();
				let required_args = field.get_required_args();

				let update = if field.is_ref() {
					quote! { #new_reference = Some(*#reference); }
				} else {
					quote! { #new_reference = Some(#reference); }
				};

				if field.is_option() {
					quote! {
						let mut #new_reference: #wrapper_final_type = None;
						if let Some(mut #unwrapped) = #wrapper_reference.take() {
							#(#operations)*
							#update
						}
					}
				} else {
					let code = &required_args.code;
					let message = &required_args.message;

					quote! {
					  let mut #new_reference: #wrapper_final_type = None;
					  if let Some(mut #unwrapped) = #wrapper_reference.take() {
							#(#operations)*
							#update
						} else {
						  let error = ValidationError::builder()
								.with_field(#field_name)
								.as_simple(#code)
								.with_message(#message)
								.build();

							append_error(&mut errors, error.into(), failure_mode, #field_name);
							if should_fail_fast(&errors, failure_mode, #field_name) {
								return Err(errors);
						  }
						}
					}
				}
			})
			.collect()
	}

	pub fn commit(&self) -> TokenStream {
		let commits: Vec<TokenStream> = self
			.0
			.iter()
			.clone()
			.map(|field| {
				let reference = field.get_reference();
				let name = field.get_name();
				let field_name = Ident::new(&name.value(), Span::call_site());

				if field.is_option() {
					quote! {
					  #field_name: #reference,
					}
				} else {
					#[rustfmt::skip]
					let result = quote! {
						#field_name: #reference.ok_or_else(|| {
						  let error = ValidationError::builder()
							  .with_field(#name)
							  .as_simple("unreachable")
							  .with_message("field missing after successful required validation check")
							  .build();

							let mut errors = ValidationErrors::new();
							append_error(&mut errors, error.into(), failure_mode, #name);

							errors
						})?,
					};

					result
				}
			})
			.collect();

		#[rustfmt::skip]
		let commit = quote! {
      Ok(Self { #(#commits)* })
		};

		commit
	}
}
