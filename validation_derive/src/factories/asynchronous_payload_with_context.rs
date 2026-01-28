use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::failure_mode::get_failure_mode_boilerplate,
		core::AbstractValidationFactory,
		extensions::payloads::get_async_payload_with_context_extensions,
		others::{payloads::PayloadsCodeFactory, wrappers::WrapperFactory},
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested,
};
use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Type, parse::ParseStream};

pub struct AsyncPayloadWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
	wrapper_factory: WrapperFactory,
}

impl<'a> AsyncPayloadWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
			wrapper_factory: WrapperFactory::default(),
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncPayloadWithContextFactory<'a> {
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
		let context_type = self.context_type;

		let (wrapper_struct, wrapper_ident) = self.wrapper_factory.create(struct_name, &fields);
		let mut code_factory = PayloadsCodeFactory(&mut fields);

		let extensions = get_async_payload_with_context_extensions(
			self.struct_name,
			attributes,
			&wrapper_ident,
			self.context_type,
			imports,
		);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().create();

		let failure_mode = get_failure_mode_boilerplate(attributes);

		#[rustfmt::skip]
		let result = quote! {
		  #wrapper_struct

		  const _: () = {
				#imports

  			#[async_trait]
  			impl AsyncValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
         	async fn async_validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
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
   			impl SpecificAsyncValidateAndParseWithContext for #struct_name {
          type Wrapper = #wrapper_ident;
          type Context = #context_type;
         	async fn specific_async_validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
       			<#struct_name as AsyncValidateAndParseWithContext<#wrapper_ident, #context_type>>::async_validate_and_parse_with_context(wrapper, context).await
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
		let (field_type, wrapper_type, nested_code) = get_nested(input);
		let context_type = self.context_type;

		if wrapper_type.is_none() {
			emit_error!(input.span(), "needs the wrapper type");
		}

		if field.is_ref() {
			field.set_is_ref(false);
			#[rustfmt::skip]
  		let result = quote! {
  		  let mut #new_reference = #field_type::default();
  			let result = if can_continue(&errors, failure_mode, #field_name) {
          <#field_type as AsyncValidateAndParseWithContext<#wrapper_type, #context_type>>::async_validate_and_parse_with_context(*#reference, context).await
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
  			}
  		};

			result
		} else {
			field.set_is_ref(false);
			#[rustfmt::skip]
  		let result = quote! {
  		  let mut #new_reference = #field_type::default();
  			let result = if can_continue(&errors, failure_mode, #field_name) {
          <#field_type as AsyncValidateAndParseWithContext<#wrapper_type, #context_type>>::async_validate_and_parse_with_context(#reference, context).await
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
  			}
  		};

			result
		}
	}
}
