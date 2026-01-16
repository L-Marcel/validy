use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	attributes::ValidationAttributes,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, modifications::get_async_modification_factory_boilerplates,
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
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);

		let struct_name = self.struct_name;

		let mut code_factory = ModificationsCodeFactory(&mut fields);
		let extensions = get_async_modification_extensions(self.struct_name, attributes, imports);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().build();

		let boilerplates = get_async_modification_factory_boilerplates(struct_name);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidateAndModificate for #struct_name {
  			  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
  					let mut errors = Vec::<ValidationError>::new();

  				  #(#operations)*

  				  if errors.is_empty() {
  						#commit
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
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let (field_type, _) = get_nested_type(input);

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndModificate>::async_validate_and_modificate(&mut #new_reference).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
