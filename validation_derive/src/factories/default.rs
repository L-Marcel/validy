use crate::{
	Output, extractors::ident::get_ident_from_nested_meta, factories::core::AbstractValidationFactory,
	import_async_trait, import_validation,
};
use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, meta::ParseNestedMeta, spanned::Spanned};

pub struct ValidationFactory<'a> {
	name: &'a Ident,
}

impl<'a> ValidationFactory<'a> {
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
			  span, "custom need a function";
				help = "#[validate(custom(my_function))] pub name: String";
				note = "fn my_function(name: &str) -> Result<(), ValidationError>"
			);
		}

		quote! {
			if let Err(e) = #function(&self.#field_name) {
			  errors.push(e);
			}
		}
	}
}

impl<'a> AbstractValidationFactory for ValidationFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let operations = &operations;

		quote! {
		  use #import;
		  use #async_trait_import;

		  impl Validation for #name {
			  fn validate(&self) -> Result<(), ValidationErrors> {
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

		  impl<C> ValidationWithContext<C> for #name {
			  fn validate_with_context(&self, context: &C) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }

			#[async_trait]
		  impl AsyncValidation for #name {
			  async fn async_validate(&self) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }

			#[async_trait]
		  impl<C> AsyncValidationWithContext<C> for #name {
			  async fn async_validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }
		}
		.into()
	}

	fn meta_is_custom(&self, meta: &ParseNestedMeta<'_>) -> bool {
		meta.path.is_ident("custom")
	}

	fn create_custom(&self, field_name: &Option<Ident>, meta: ParseNestedMeta<'_>) -> TokenStream {
		let function = get_ident_from_nested_meta(&meta);
		ValidationFactory::create_custom_from(field_name, &meta, function)
	}

	fn create_nested(&self, field_name: &Option<Ident>, field_type: &Type) -> TokenStream {
		quote! {
		  if let Err(e) = <#field_type as Validation>::validate(&self.#field_name) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					stringify!(#field_name),
				)));
		  }
		}
	}
}
