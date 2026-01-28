use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Fields, Ident, Meta, Path, Token, parse_quote, punctuated::Punctuated};

use crate::{attributes::ValidationAttributes, fields::FieldAttributes, get_fields};

#[derive(Default)]
pub struct WrapperFactory {
	struct_derives: Vec<Path>,
	struct_attributes: Vec<Attribute>,
	fields_attributes: HashMap<String, Vec<Attribute>>,
}

impl WrapperFactory {
	pub fn from(input: &DeriveInput, attributes: &ValidationAttributes) -> Self {
		let fields = get_fields(input);
		let struct_attributes = get_attributes_by_structs(&input.attrs);
		let fields_attributes = get_attributes_by_fields(fields);
		let mut struct_derives = get_derives_by_structs(&input.attrs);

		let native_derives = if attributes.multipart {
			vec![parse_quote!(Debug), parse_quote!(Default)]
		} else {
			vec![
				parse_quote!(Debug),
				parse_quote!(Default),
				parse_quote!(::serde::Deserialize),
			]
		};

		struct_derives.extend(native_derives);

		WrapperFactory {
			struct_derives,
			struct_attributes,
			fields_attributes,
		}
	}

	pub fn create<'a>(&self, name: &'a Ident, fields: &'a [FieldAttributes]) -> (TokenStream, Ident) {
		let struct_derives = &self.struct_derives;
		let struct_attributes = &self.struct_attributes;

		let wrapper_ident = format_ident!("{}Wrapper", name);
		let field_declarations: Vec<TokenStream> = fields
			.iter()
			.clone()
			.map(|field| {
				let name = field.get_name();
				let field_type = field.get_initial_type();
				let field_name = Ident::new(&name.value(), Span::call_site());
				let field_attributes: Vec<&Attribute> = self
					.fields_attributes
					.get(&name.value())
					.into_iter()
					.flatten()
					.collect();

				quote! {
				  #(#field_attributes)*
				  pub #field_name: #field_type,
				}
			})
			.collect();

		#[rustfmt::skip]
		let wrapper_struct = quote! {
  		#[derive(#(#struct_derives),*)]
      #(#struct_attributes)*
  		pub struct #wrapper_ident {
  		  #(#field_declarations)*
  		}
		};

		(wrapper_struct, wrapper_ident)
	}
}

static SUPPORTED_ATTRIBUTES: &[&str] = &["form_data", "field", "serde"];
fn get_attributes_for_fields(attributes: &[Attribute]) -> Vec<Attribute> {
	attributes
		.iter()
		.filter_map(|attribute| {
			SUPPORTED_ATTRIBUTES.iter().find_map(|other| {
				if attribute.path().is_ident(other) {
					Some(attribute.clone())
				} else {
					None
				}
			})
		})
		.collect()
}

fn get_attributes_by_fields(fields: &Fields) -> HashMap<String, Vec<Attribute>> {
	fields
		.iter()
		.enumerate()
		.fold(HashMap::new(), |mut accumulator, (index, field)| {
			let name: String = match &field.ident {
				Some(ident) => ident.to_string(),
				None => index.to_string(),
			};

			let attributes = get_attributes_for_fields(&field.attrs);

			if !attributes.is_empty() {
				accumulator.insert(name, attributes);
			}

			accumulator
		})
}

static SUPPORTED_STRUCT_ATTRIBUTES: &[&str] = &["serde", "try_from_multipart"];
fn get_attributes_by_structs(attributes: &[Attribute]) -> Vec<Attribute> {
	attributes
		.iter()
		.filter_map(|attribute| {
			SUPPORTED_STRUCT_ATTRIBUTES.iter().find_map(|other| {
				if attribute.path().is_ident(other) {
					Some(attribute.clone())
				} else {
					None
				}
			})
		})
		.collect()
}

static NATIVE_DERIVES: &[&str] = &["Debug", "Default", "Deserialize"];
fn get_derives_by_structs(attributes: &[Attribute]) -> Vec<Path> {
	let mut derives = Vec::new();

	for attribute in attributes {
		if attribute.path().is_ident("wrapper_derive")
			&& let Ok(nested) = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
		{
			for meta in nested {
				if let Meta::Path(path) = meta
					&& !NATIVE_DERIVES.iter().all(|native| path.is_ident(native))
				{
					derives.push(path);
				}
			}
		}
	}

	derives
}
