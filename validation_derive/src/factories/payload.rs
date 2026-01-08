use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
	imports::import_serde_deserialize,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

pub struct PayloadFactory<'a> {
	name: &'a Ident,
}

impl<'a> PayloadFactory<'a> {
	pub fn new(name: &'a Ident) -> Self {
		Self { name }
	}
}

impl<'a> AbstractValidationFactory for PayloadFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let serde_deserialize_import = import_serde_deserialize();
		let import = import_validation();

		let name = self.name;

		let field_declarations: Vec<TokenStream> = fields
			.iter()
			.clone()
			.map(|field| {
				let name = field.get_name();
				let field_type = field.get_initial_type();
				let field_name = Ident::new(&name.value(), Span::call_site());

				quote! {
				  pub #field_name: #field_type,
				}
			})
			.collect();

		let operations: Vec<TokenStream> = fields.iter_mut().map(|field| field.get_operations()).collect();

		let commits: Vec<TokenStream> = fields
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
					quote! {
					  #field_name: #reference.ok_or_else(|| {
						  let error = ValidationError::builder()
							  .with_field(#name)
							  .as_simple("unreachable")
							  .with_message("field missing after successful required validation check")
							  .build();

						  let errors: Vec<ValidationError> = vec![error.into()];

						  let map: ValidationErrors = errors
							  .into_iter()
							  .map(|e| match e {
								  ValidationError::Node(e) => (e.field.clone().into(), ValidationError::Node(e)),
								  ValidationError::Leaf(e) => (e.field.clone().into(), ValidationError::Leaf(e)),
							  })
							  .collect();

						  map
						})?
					}
				}
			})
			.collect();

		let wrapper_ident = format_ident!("{}Wrapper", name);

		#[rustfmt::skip]
		let result = quote! {
		  use #import;
			use #async_trait_import;
			use #serde_deserialize_import;

      #[derive(Deserialize)]
      struct #wrapper_ident {
        #(#field_declarations)*
      }

      impl ValidateAndParse<#wrapper_ident> for #name {
        fn validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
          let mut errors = Vec::<ValidationError>::new();

          #(#operations)*

          if errors.is_empty() {
            Ok(Self { #(#commits)* })
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

		  impl<C> ValidateAndParseWithContext<#wrapper_ident, C> for #name {
			  fn validate_and_parse_with_context(___wrapper: &#wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
					<#name as ValidateAndParse<#wrapper_ident>>::validate_and_parse(___wrapper)
				}
			}

			#[async_trait]
			impl AsyncValidateAndParse<#wrapper_ident> for #name {
			  async fn async_validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
					<#name as ValidateAndParse<#wrapper_ident>>::validate_and_parse(___wrapper)
				}
			}

			#[async_trait]
			impl<C> AsyncValidateAndParseWithContext<#wrapper_ident, C> for #name {
			  async fn async_validate_and_parse_with_context(___wrapper: &#wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
					<#name as ValidateAndParse<#wrapper_ident>>::validate_and_parse(___wrapper)
				}
		  }
		};

		result.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let wrapper_ident = format_ident!("{}Wrapper", field_type.to_token_stream().to_string());

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as ValidateAndParse<#wrapper_ident>>::validate_and_parse(&#new_reference) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
