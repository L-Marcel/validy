use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::failure_mode::get_failure_mode_boilerplate, core::AbstractValidationFactory,
		extensions::modifications::get_async_modification_with_context_extensions,
		utils::modifications::ModificationsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested_type,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, parse::ParseStream};

pub struct AsyncModificationWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> AsyncModificationWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncModificationWithContextFactory<'a> {
	fn create(
		&self,
		mut fields: Vec<FieldAttributes>,
		attributes: &ValidationAttributes,
		imports: &RefCell<ImportsSet>,
	) -> Output {
		imports.borrow_mut().add(Import::ValidyCore);
		imports.borrow_mut().add(Import::ValidySettings);
		imports.borrow_mut().add(Import::ValidyHelpers);
		imports.borrow_mut().add(Import::AsyncTrait);

		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = ModificationsCodeFactory(&mut fields);
		let extensions =
			get_async_modification_with_context_extensions(self.struct_name, attributes, self.context_type, imports);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().build();

		let failure_mode = get_failure_mode_boilerplate(attributes);

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidateAndModificateWithContext<#context_type> for #struct_name {
  			  async fn async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
     				let mut errors = ValidationErrors::new();
            let failure_mode = #failure_mode;

  				  #(#operations)*

  				  if errors.is_empty() {
  						#commit
  				  } else {
      				Err(errors)
  				  }
  			  }
  		  }

   			#[async_trait]
   		  impl SpecificAsyncValidateAndModificateWithContext for #struct_name {
          type Context = #context_type;
   			  async fn specific_async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
            <#struct_name as AsyncValidateAndModificateWithContext<#context_type>>::async_validate_and_modificate_with_context(self, context).await
   			  }
   		  }

        #extensions
			};
		};

		result.into()
	}

	fn create_nested(&self, input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let (field_type, _) = get_nested_type(input);
		let context_type = self.context_type;

		field.set_is_ref(false);

		#[rustfmt::skip]
		let result = quote! {
		  let mut #new_reference = #reference.clone();
		  if can_continue(&errors, failure_mode, #field_name) && let Err(e) = <#field_type as AsyncValidateAndModificateWithContext<#context_type>>::async_validate_and_modificate_with_context(&mut #new_reference, context).await {
				let error = NestedValidationError::from(
					e,
					#field_name,
				);

			  append_error(&mut errors, error.into(), failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
