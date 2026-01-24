use std::cell::RefCell;

use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, meta::ParseNestedMeta, parse::ParseStream};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	core::{get_operation_by_attr_macro, get_special_by_attr_macro, get_validation_by_attr_macro},
	factories::core::AbstractValidationFactory,
	fields::FieldAttributes,
	primitives::commons::{ArgParser, extract_inner_type, parse_attrs, remove_parens},
};

#[derive(Default)]
pub struct ForEachArgs {
	pub from_collection: Option<Type>,
	pub to_collection: Option<Type>,
	pub from_item: Option<Type>,
}

impl ForEachArgs {
	pub fn update_from_type(&mut self, current_type: &Type, field: &mut FieldAttributes) {
		if self.from_collection.is_none() {
			self.from_collection = Some(current_type.clone());
		}

		if self.from_item.is_none()
			&& let Some(current_type) = self.from_collection.as_ref()
		{
			self.from_item = extract_inner_type(current_type);
		}

		if self.to_collection.is_none()
			&& let Some(current_type) = self.from_collection.as_ref()
		{
			self.to_collection = Some(current_type.clone());
		}

		if let Some(current_type) = self.to_collection.as_ref() {
			field.set_current_type(current_type);
		}
	}

	pub fn update_from_item_type(&mut self, field: &mut FieldAttributes) {
		if let Some(current_type) = self.from_item.as_ref() {
			field.set_current_type(current_type);
		}
	}
}

impl ArgParser for ForEachArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["from_item", "to_collection", "from_collection"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"from_item" => self.from_item = Some(input.parse()?),
			"to_collection" => self.to_collection = Some(input.parse()?),
			"from_collection" => self.from_collection = Some(input.parse()?),
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_for_each(
	factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	let mut operations = Vec::<TokenStream>::new();
	let reference = field.get_reference();
	field.enter_scope();
	let is_ref = field.is_ref();
	field.set_is_ref(true);

	let item_reference = field.get_reference();
	let mut args = ForEachArgs::default();
	let current_type = field.get_current_type().clone();
	args.update_from_type(&current_type, field);

	if attributes.modify {
		field.increment_modifications();
		let new_reference = field.get_reference();
		field.set_is_ref(false);

		operations.push(quote! {
		  let mut #new_reference = #item_reference.clone();
		});
	};

	let _ = meta.parse_nested_meta(|meta| {
		if meta.path.is_ident("config") {
			let content = remove_parens(meta.input);
			args = match content {
				Ok(content) => parse_attrs(&content)
					.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
					.unwrap_or_default(),
				Err(_) => ForEachArgs::default(),
			};

			args.update_from_item_type(field);
		} else if meta.path.is_ident("validate") {
			let _ = meta.parse_nested_meta(|meta| {
				let validation = get_validation_by_attr_macro(factory, meta, field, attributes, imports);
				operations.push(validation.clone());
				Ok(())
			});
		} else if meta.path.is_ident("modify") {
			let _ = meta.parse_nested_meta(|meta| {
				let operation = get_operation_by_attr_macro(factory, meta, field, attributes, imports);
				operations.push(operation.clone());
				Ok(())
			});
		} else if meta.path.is_ident("special") {
			let _ = meta.parse_nested_meta(|meta| {
				let operation = get_special_by_attr_macro(factory, meta, field, attributes, imports);
				operations.push(operation.clone());
				Ok(())
			});
		}
		Ok(())
	});

	args.update_from_type(&current_type, field);

	let final_item_reference = field.get_reference();
	field.exit_scope();
	field.increment_modifications();

	if attributes.modify && !attributes.payload {
		let new_reference = field.get_reference();
		let to_collection = args.to_collection;

		let iterator_source = if is_ref {
			quote! { ::std::mem::take(#reference) }
		} else {
			quote! { ::std::mem::take(&#reference) }
		};

		#[rustfmt::skip]
		let result = quote! {
		  let mut #new_reference: #to_collection = Default::default();
		  for #item_reference in #iterator_source.into_iter() {
				#(#operations)*

				Extend::extend(
					&mut #new_reference,
					::std::iter::once(#final_item_reference.clone())
				);
		  }
		};

		result
	} else if attributes.payload {
		let new_reference = field.get_reference();
		let to_collection = args.to_collection;

		#[rustfmt::skip]
		let result = quote! {
		  let mut #new_reference: #to_collection = Default::default();
		  for #item_reference in #reference.into_iter() {
				#(#operations)*

				Extend::extend(
					&mut #new_reference,
					::std::iter::once(#final_item_reference.clone())
				);
		  }
		};

		result
	} else {
		let iterator_source = if is_ref {
			quote! { #reference }
		} else {
			quote! { &#reference }
		};

		#[rustfmt::skip]
		let result = quote! {
			for ref #item_reference in #iterator_source {
				#(#operations)*
		  }
		};

		result
	}
}
