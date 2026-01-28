use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{failure_mode::get_failure_mode_boilerplate, payloads::get_async_payload_factory_boilerplates},
		core::AbstractValidationFactory,
		extensions::payloads::get_async_payload_extensions,
		others::{payloads::PayloadsCodeFactory, wrappers::WrapperFactory},
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested,
};
use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse::ParseStream};

pub struct AsyncPayloadFactory<'a> {
	struct_name: &'a Ident,
	wrapper_factory: WrapperFactory,
}

impl<'a> AsyncPayloadFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self {
			struct_name,
			wrapper_factory: WrapperFactory::default(),
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncPayloadFactory<'a> {
	fn init(&mut self, input: &DeriveInput, attributes: &ValidationAttributes) {
		self.wrapper_factory = WrapperFactory::from(input, attributes);
	}

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

		let (wrapper_struct, wrapper_ident) = self.wrapper_factory.create(struct_name, &fields);
		let mut code_factory = PayloadsCodeFactory(&mut fields);

		let extensions = get_async_payload_extensions(self.struct_name, attributes, &wrapper_ident, imports);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().create();

		let boilerplates = get_async_payload_factory_boilerplates(struct_name, &wrapper_ident);
		let failure_mode = get_failure_mode_boilerplate(attributes);

		#[rustfmt::skip]
		let result = quote! {
			#wrapper_struct

		  const _: () = {
				#imports

  			#[async_trait]
  			impl AsyncValidateAndParse<#wrapper_ident> for #struct_name {
         	async fn async_validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
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
   			impl SpecificAsyncValidateAndParse for #struct_name {
          type Wrapper = #wrapper_ident;
         	async fn specific_async_validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
       			<#struct_name as AsyncValidateAndParse<#wrapper_ident>>::async_validate_and_parse(wrapper).await
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
		let (field_type, wrapper_type, nested_code) = get_nested(input);

		if wrapper_type.is_none() {
			emit_error!(input.span(), "needs the wrapper type");
		}

		if field.is_ref() {
			field.set_is_ref(false);
			#[rustfmt::skip]
  		let result = quote! {
  		  let mut #new_reference = #field_type::default();
  			let result = if can_continue(&errors, failure_mode, #field_name) {
  	      <#field_type as AsyncValidateAndParse<#wrapper_type>>::async_validate_and_parse(*#reference).await
  			} else {
  			  Ok(#field_type::default())
  			};

  			match result {
  			  Ok(value) => #new_reference = value,
  				Err(e) =>  {
    				let error = NestedValidationError::from_with_code(
     					e,
     					#field_name,
              #nested_code
    				);

  			    append_error(&mut errors, error.into(), failure_mode, #field_name);
            if should_fail_fast(&errors, failure_mode, #field_name) {
         			return Err(errors);
         	  }
  			  },
  			};
  		};

			result
		} else {
			field.set_is_ref(false);
			#[rustfmt::skip]
  		let result = quote! {
  		  let mut #new_reference = #field_type::default();
  			let result = if can_continue(&errors, failure_mode, #field_name) {
  	      <#field_type as AsyncValidateAndParse<#wrapper_type>>::async_validate_and_parse(#reference).await
  			} else {
  			  Ok(#field_type::default())
  			};

  			match result {
  			  Ok(value) => #new_reference = value,
  				Err(e) =>  {
    				let error = NestedValidationError::from_with_code(
     					e,
     					#field_name,
              #nested_code
    				);

  			    append_error(&mut errors, error.into(), failure_mode, #field_name);
            if should_fail_fast(&errors, failure_mode, #field_name) {
         			return Err(errors);
         	  }
  			  },
  			};
  		};

			result
		}
	}
}
