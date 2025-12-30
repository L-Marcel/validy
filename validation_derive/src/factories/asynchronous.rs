use crate::{
	Output,
	extractors::ident::get_ident_from_nested_meta,
	factories::{core::AbstractValidationFactory, default::ValidationFactory},
	import_async_trait, import_validation,
};
use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, meta::ParseNestedMeta, spanned::Spanned};

pub struct AsyncValidationFactory<'a> {
	name: &'a Ident,
}

impl<'a> AsyncValidationFactory<'a> {
	pub fn new(name: &'a Ident) -> Self {
		Self { name }
	}

	pub fn create_custom_from(
		field_name: &Option<Ident>,
		meta: &ParseNestedMeta<'_>,
		function: Option<Ident>,
	) -> TokenStream {
		if function.is_none() {
			let span = meta.path.span();
			emit_error!(
			  span, "async_custom need a function";
				help = "#[validate(async_custom(my_function))] pub name: String";
				note = "async fn my_function(name: &str) -> Result<(), ValidationError>"
			);
		}

		quote! {
			if let Err(e) = #function(&self.#field_name).await {
			  errors.push(e);
			}
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncValidationFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let operations = &operations;

		quote! {
		  use #import;
		  use #async_trait_import;

			#[async_trait]
		  impl AsyncValidation for #name {
			  async fn async_validate(&self) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
					  Ok(())
				  } else {
						let map: ValidationErrors = errors
							.into_iter()
							.map(|e| match e {
								ValidationError::Node(e) => (e.field.clone(), ValidationError::Node(e)),
								ValidationError::Leaf(e) => (e.field.clone(), ValidationError::Leaf(e)),
							})
							.collect();

					  Err(map)
				  }
			  }
		  }

			#[async_trait]
		  impl<C> AsyncValidationWithContext<C> for #name {
			  async fn async_validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
				  self.async_validate().await
			  }
		  }
		}
		.into()
	}

	fn meta_is_custom(&self, meta: &ParseNestedMeta<'_>) -> bool {
		meta.path.is_ident("async_custom") || meta.path.is_ident("custom")
	}

	fn create_custom(&self, field_name: &Option<Ident>, meta: ParseNestedMeta<'_>) -> TokenStream {
		let function = get_ident_from_nested_meta(&meta);

		match meta {
			m if meta.path.is_ident("custom") => ValidationFactory::create_custom_from(field_name, &m, function),
			m => AsyncValidationFactory::create_custom_from(field_name, &m, function),
		}
	}

	fn create_nested(&self, field_name: &Option<Ident>, field_type: &Type) -> TokenStream {
		quote! {
		  if let Err(e) = <#field_type as AsyncValidation>::validate(&self.#field_name).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					stringify!(#field_name),
				)));
		  }
		}
	}
}
