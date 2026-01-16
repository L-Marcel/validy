use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	factories::extensions::axum::payloads::{
		get_async_payload_axum_extension, get_async_payload_with_context_axum_extension,
	},
};

pub fn get_payload_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Ident,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_payload_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_payload_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Ident,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_payload_with_context_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_async_payload_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Ident,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_payload_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_async_payload_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Ident,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_payload_with_context_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}
