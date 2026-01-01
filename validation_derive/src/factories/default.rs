use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct ValidationFactory<'a> {
	name: &'a Ident,
}

impl<'a> ValidationFactory<'a> {
	pub fn new(name: &'a Ident) -> Self {
		Self { name }
	}
}

impl<'a> AbstractValidationFactory for ValidationFactory<'a> {
	fn create(&self, operations: Vec<TokenStream>, _: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = &self.name;
		let operations = &operations;

		quote! {
		  use #import;
		  use #async_trait_import;

		  impl Validate for #name {
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

		  impl<C> ValidateWithContext<C> for #name {
			  fn validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }

			#[async_trait]
		  impl AsyncValidate for #name {
			  async fn async_validate(&self) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }

			#[async_trait]
		  impl<C> AsyncValidateWithContext<C> for #name {
			  async fn async_validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
				  self.validate()
			  }
		  }

			#[async_trait]
		  impl AsyncValidateAndModificate for #name {
			  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
					 self.validate()
				}
			}

			#[async_trait]
		  impl<C> AsyncValidateAndModificateWithContext<C> for #name {
			  async fn async_validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
					self.validate()
				}
			}

		  impl ValidateAndModificate for #name {
			  fn validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
					self.validate()
				}
		  }

			impl<C> ValidateAndModificateWithContext<C> for #name {
			  fn validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
					self.validate()
				}
			}
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();

		quote! {
		  if let Err(e) = <#field_type as Validate>::validate(&#reference) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
