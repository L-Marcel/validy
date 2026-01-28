use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
	AttrStyle, Attribute, DeriveInput, Fields, Ident, Meta, Path, Token, parse_quote,
	punctuated::Punctuated,
	token::{Bracket, Pound},
};

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
			vec![
				parse_quote!(Default),
				parse_quote!(::axum_typed_multipart::TryFromMultipart),
			]
		} else {
			vec![parse_quote!(Default), parse_quote!(::serde::Deserialize)]
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

static NATIVE_FIELD_ATTRIBUTES: &[&str] = &["wrapper_attribute", "validate", "modificate", "parse", "special"];
fn get_attributes_for_fields(attributes: &[Attribute]) -> Vec<Attribute> {
	let mut fields_attributes = Vec::new();

	for attribute in attributes {
		if attribute.path().is_ident("wrapper_attribute")
			&& let Ok(nested) = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
		{
			for meta in nested {
				if !NATIVE_FIELD_ATTRIBUTES
					.iter()
					.any(|native| meta.path().is_ident(native))
				{
					fields_attributes.push(Attribute {
						pound_token: Pound::default(),
						style: AttrStyle::Outer,
						bracket_token: Bracket::default(),
						meta,
					});
				}
			}
		}
	}

	fields_attributes
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

static NATIVE_STRUCT_ATTRIBUTES: &[&str] = &[
	"derive",
	"wrapper_derive",
	"wrapper_attribute",
	"validate",
	"modificate",
	"parse",
];
fn get_attributes_by_structs(attributes: &[Attribute]) -> Vec<Attribute> {
	let mut struct_attributes = Vec::new();

	for attribute in attributes {
		if attribute.path().is_ident("wrapper_attribute")
			&& let Ok(nested) = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
		{
			for meta in nested {
				if !NATIVE_STRUCT_ATTRIBUTES
					.iter()
					.any(|native| meta.path().is_ident(native))
				{
					struct_attributes.push(Attribute {
						pound_token: Pound::default(),
						style: AttrStyle::Outer,
						bracket_token: Bracket::default(),
						meta,
					});
				}
			}
		}
	}

	struct_attributes
}

static NATIVE_DERIVES: &[&str] = &["Default", "Deserialize", "TryFromMultipart"];
fn get_derives_by_structs(attributes: &[Attribute]) -> Vec<Path> {
	let mut derives = Vec::new();

	for attribute in attributes {
		if attribute.path().is_ident("wrapper_derive")
			&& let Ok(nested) = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
		{
			for meta in nested {
				if let Meta::Path(path) = meta
					&& !NATIVE_DERIVES.iter().any(|native| path.is_ident(native))
				{
					derives.push(path);
				}
			}
		}
	}

	derives
}
