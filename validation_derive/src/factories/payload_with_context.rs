use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{
			failure_mode::get_failure_mode_boilerplate, payloads::get_payload_with_context_factory_boilerplates,
		},
		core::AbstractValidationFactory,
		extensions::payloads::get_payload_with_context_extensions,
		utils::payloads::PayloadsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested_type,
};
use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use std::cell::RefCell;
use syn::{Ident, Type, parse::ParseStream};

pub struct PayloadWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> PayloadWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for PayloadWithContextFactory<'a> {
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

		let mut code_factory = PayloadsCodeFactory(&mut fields);
		let (wrapper_struct, wrapper_ident) = code_factory.wrapper(struct_name);
		let extensions = get_payload_with_context_extensions(
			self.struct_name,
			attributes,
			&wrapper_ident,
			self.context_type,
			imports,
		);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().build();

		let boilerplates = get_payload_with_context_factory_boilerplates(struct_name, &wrapper_ident, context_type);
		let failure_mode = get_failure_mode_boilerplate(attributes);

		#[rustfmt::skip]
		let result = quote! {
			#wrapper_struct

			const _: () = {
  		  #imports

  			impl ValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
         	fn validate_and_parse_with_context(wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
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

        impl SpecificValidateAndParseWithContext for #struct_name {
          type Wrapper = #wrapper_ident;
          type Context = #context_type;
     			fn specific_validate_and_parse_with_context(wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
  					<#struct_name as ValidateAndParseWithContext<#wrapper_ident, #context_type>>::validate_and_parse_with_context(wrapper, context)
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
		let (field_type, wrapper_type) = get_nested_type(input);
		let context_type = self.context_type;

		if wrapper_type.is_none() {
			emit_error!(input.span(), "needs the wrapper type");
		}

		field.set_is_ref(false);

		#[rustfmt::skip]
		let result = quote! {
			let mut #new_reference = #field_type::default();

			let result = if can_continue(&errors, failure_mode, #field_name) {
        <#field_type as ValidateAndParseWithContext<#wrapper_type, #context_type>>::validate_and_parse_with_context(#reference.clone(), context)
			} else {
        Ok(#field_type::default())
			};

			match result {
			  Ok(value) => #new_reference = value,
				Err(e) =>  {
  				let error = NestedValidationError::from(
  					e,
  					#field_name,
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
