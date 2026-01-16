use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	factories::extensions::axum::modifications::{
		get_async_modification_axum_extension, get_async_modification_with_context_axum_extension,
	},
};

pub fn get_modification_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_modification_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_modification_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_modification_with_context_axum_extension(
		struct_name,
		attributes,
	)];
	quote! { #(#extensions)* }
}

pub fn get_async_modification_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_modification_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_async_modification_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_modification_with_context_axum_extension(
		struct_name,
		attributes,
	)];
	quote! { #(#extensions)* }
}
