use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct AsyncModificationWithContextFactory<'a> {
	name: &'a Ident,
	context: &'a Type,
}

impl<'a> AsyncModificationWithContextFactory<'a> {
	pub fn new(name: &'a Ident, context: &'a Type) -> Self {
		Self { name, context }
	}
}

impl<'a> AbstractValidationFactory for AsyncModificationWithContextFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>, fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let context = &self.context;
		let operations = &operations;

		let commits = fields
			.into_iter()
			.filter(|field| field.get_modifications() > 0)
			.map(|field| {
				let reference = field.get_reference();
				let original_reference = field.get_original_reference();
				quote! {
				  #original_reference = #reference;
				}
			});

		quote! {
		  use #import;
		  use #async_trait_import;

			#[async_trait]
		  impl AsyncValidateAndModificateWithContext<#context> for #name {
			  async fn async_validate_and_modificate_with_context(&mut self, context: &#context) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
						#(#commits)*
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
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context = &self.context;

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndModificateWithContext<#context>>::async_validate_and_modificate_with_context(&mut #new_reference, context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
