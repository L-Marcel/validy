use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	factories::extensions::axum::defaults::{
		get_async_default_axum_extension, get_async_default_with_context_axum_extension,
	},
};

pub fn get_default_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_default_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_default_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_default_with_context_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_async_default_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_default_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}

pub fn get_async_default_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_default_with_context_axum_extension(struct_name, attributes)];
	quote! { #(#extensions)* }
}
