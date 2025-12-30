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

pub struct ValidationWithContextFactory<'a> {
	name: &'a Ident,
	context: &'a Type,
}

impl<'a> ValidationWithContextFactory<'a> {
	pub fn new(name: &'a Ident, context: &'a Type) -> Self {
		Self { name, context }
	}

	pub fn create_custom_from(
		field_name: &Option<Ident>,
		meta: &ParseNestedMeta<'_>,
		function: Option<Ident>,
	) -> TokenStream {
		if function.is_none() {
			let span = meta.path.span();
			emit_error!(
			  span, "custom_with_context need a function";
				help = "#[validate(custom_with_context(my_function))] pub name: String";
				note = "fn my_function(name: &str, context: &C) -> Result<(), ValidationError>"
			);
		}

		quote! {
			if let Err(e) = #function(&self.#field_name, context) {
			  errors.push(e);
			}
		}
	}
}

impl<'a> AbstractValidationFactory for ValidationWithContextFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let context = &self.context;
		let operations = &operations;

		quote! {
		  use #import;
		  use #async_trait_import;

		  impl ValidationWithContext<#context> for #name {
			  fn validate_with_context(&self, context: &#context) -> Result<(), ValidationErrors> {
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
		  impl AsyncValidationWithContext<#context> for #name
		  where
				#context: Send + Sync,
		  {
			  async fn async_validate_with_context(&self, context: &#context) -> Result<(), ValidationErrors> {
				  self.validate_with_context(context)
			  }
		  }
		}
		.into()
	}

	fn meta_is_custom(&self, meta: &ParseNestedMeta<'_>) -> bool {
		meta.path.is_ident("custom_with_context") || meta.path.is_ident("custom")
	}

	fn create_custom(&self, field_name: &Option<Ident>, meta: ParseNestedMeta<'_>) -> TokenStream {
		let function = get_ident_from_nested_meta(&meta);

		match meta {
			m if meta.path.is_ident("custom") => ValidationFactory::create_custom_from(field_name, &m, function),
			m => ValidationWithContextFactory::create_custom_from(field_name, &m, function),
		}
	}

	fn create_nested(&self, field_name: &Option<Ident>, field_type: &Type) -> TokenStream {
		let context = &self.context;

		quote! {
		  if let Err(e) = <#field_type as ValidationWithContext<#context>>::validate_with_context(&self.#field_name, &context) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					stringify!(#field_name),
				)));
			}
		}
	}
}
