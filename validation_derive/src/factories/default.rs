use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{commons::get_throw_errors_boilerplate, defaults::get_default_factory_boilerplates},
		core::AbstractValidationFactory,
		extensions::defaults::get_default_extensions,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
	primitives::specials::nested::get_nested_type,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, parse::ParseStream};

pub struct ValidationFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> ValidationFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for ValidationFactory<'a> {
	fn create(
		&self,
		mut fields: Vec<FieldAttributes>,
		attributes: &ValidationAttributes,
		imports: &RefCell<ImportsSet>,
	) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);

		let struct_name = self.struct_name;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let extensions = get_default_extensions(self.struct_name, attributes, imports);

		let operations = code_factory.operations();
		let imports = imports.borrow().build();

		let boilerplates = get_default_factory_boilerplates(struct_name);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			impl Validate for #struct_name {
  				fn validate(&self) -> Result<(), ValidationErrors> {
    				let mut errors = Vec::<ValidationError>::new();

  					#(#operations)*

  					if errors.is_empty() {
  						Ok(())
  					} else {
  						#throw_errors
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
		let field_name = field.get_name();
		let (field_type, _) = get_nested_type(input);

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
