use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct ValidationWithContextFactory<'a> {
	name: &'a Ident,
	context: &'a Type,
}

impl<'a> ValidationWithContextFactory<'a> {
	pub fn new(name: &'a Ident, context: &'a Type) -> Self {
		Self { name, context }
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

	fn create_nested(&self, field: &FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context = &self.context;

		quote! {
		  if let Err(e) = <#field_type as ValidationWithContext<#context>>::validate_with_context(&#reference, &context) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
