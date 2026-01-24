use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{
			failure_mode::get_failure_mode_boilerplate, modifications::get_async_modification_factory_boilerplates,
		},
		core::AbstractValidationFactory,
		extensions::modifications::get_async_modification_extensions,
		utils::modifications::ModificationsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested_type,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, parse::ParseStream};

pub struct AsyncModificationFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> AsyncModificationFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for AsyncModificationFactory<'a> {
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

		let mut code_factory = ModificationsCodeFactory(&mut fields);
		let extensions = get_async_modification_extensions(self.struct_name, attributes, imports);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().build();

		let boilerplates = get_async_modification_factory_boilerplates(struct_name);
		let failure_mode = get_failure_mode_boilerplate(attributes);

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidateAndModificate for #struct_name {
  			  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
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

  			#boilerplates

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

		field.set_is_ref(false);

		#[rustfmt::skip]
		let result = quote! {
		  let mut #new_reference = #reference.clone();
		  if can_continue(&errors, failure_mode, #field_name) && let Err(e) = <#field_type as AsyncValidateAndModificate>::async_validate_and_modificate(&mut #new_reference).await {
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
