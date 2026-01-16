use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, defaults::get_async_default_factory_with_context_boilerplates,
		},
		core::AbstractValidationFactory,
		extensions::defaults::get_async_default_with_context_extensions,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested_type,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, parse::ParseStream};

pub struct AsyncValidationWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> AsyncValidationWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncValidationWithContextFactory<'a> {
	fn create(
		&self,
		mut fields: Vec<FieldAttributes>,
		attributes: &ValidationAttributes,
		imports: &RefCell<ImportsSet>,
	) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);

		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let extensions =
			get_async_default_with_context_extensions(self.struct_name, attributes, self.context_type, imports);

		let operations = code_factory.operations();
		let imports = imports.borrow().build();

		let boilerplates = get_async_default_factory_with_context_boilerplates(struct_name, context_type);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidateWithContext<#context_type> for #struct_name {
  			  async fn async_validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
  					let mut errors = Vec::<ValidationError>::new();

  				  #(#operations)*

  				  if errors.is_empty() {
  					  Ok(())
  				  } else {
  						#throw_errors
  				  }
  			  }
  		  }

   			#[async_trait]
   		  impl SpecificAsyncValidateWithContext for #struct_name {
          type Context = #context_type;
   			  async fn specific_async_validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
            <#struct_name as AsyncValidateWithContext<#context_type>>::async_validate_with_context(self, context).await
   			  }
   		  }

  			#boilerplates

        #extensions
			};
		};

		result.into()
	}

	fn create_nested(&self, input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let (field_type, _) = get_nested_type(input);
		let context_type = self.context_type;

		quote! {
		  if let Err(e) = <#field_type as AsyncValidateWithContext<#context_type>>::async_validate_with_context(&#reference, &context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
