use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct AsyncValidationWithContextFactory<'a> {
	name: &'a Ident,
	context: &'a Type,
}

impl<'a> AsyncValidationWithContextFactory<'a> {
	pub fn new(name: &'a Ident, context: &'a Type) -> Self {
		Self { name, context }
	}
}

impl<'a> AbstractValidationFactory for AsyncValidationWithContextFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>, _: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let context = &self.context;
		let operations = &operations;

		quote! {
		  use #import;
		  use #async_trait_import;

			#[async_trait]
		  impl AsyncValidateWithContext<#context> for #name {
			  async fn async_validate_with_context(&self, context: &#context) -> Result<(), ValidationErrors> {
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
		  impl AsyncValidateAndModificateWithContext<#context> for #name {
			  async fn async_validate_and_modificate_with_context(&mut self, context: &#context) -> Result<(), ValidationErrors> {
				  self.async_validate_with_context(context);
				}
			}
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context = &self.context;

		quote! {
		  if let Err(e) = <#field_type as AsyncValidateWithContext<#context>>::async_validate_with_context(&#reference, &context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
