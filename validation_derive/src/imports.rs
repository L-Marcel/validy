use std::collections::HashSet;

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse_str;

pub struct ImportsSet {
	set: HashSet<Import>,
}

impl ImportsSet {
	pub fn new() -> Self {
		ImportsSet { set: HashSet::new() }
	}

	pub fn add(&mut self, import: Import) {
		self.set.insert(import);
	}

	pub fn build(&self) -> TokenStream {
		let imports: Vec<TokenStream> = self
			.set
			.iter()
			.map(|import| {
				let import = match import {
					Import::ValidationFunction(function) => import_validation_functions(function),
					Import::ModificationFunction(function) => import_modification_functions(function),
					Import::ValidyCore => import_validy(),
					Import::ValidySettings => import_validy_settings(),
					Import::ValidyHelpers => import_validy_helpers(),
					Import::AsyncTrait => import_async_trait(),
					Import::TryFromMultipart => import_try_from_multipart(),
					Import::Deserialize => import_deserialize(),
					Import::Serialize => import_serialize(),
				};

				quote! { use #import; }
			})
			.collect();

		quote! { #(#imports)* }
	}

	pub fn get_derives(&self) -> TokenStream {
		let mut derives = vec![quote!(Debug), quote!(Default), quote!(::serde::Deserialize)];

		if self.set.contains(&Import::Serialize) {
			derives.push(quote!(::serde::Serialize));
		}

		if self.set.contains(&Import::TryFromMultipart) {
			derives.push(quote!(axum_typed_multipart::TryFromMultipart));
		}

		quote! {
			#[derive(#(#derives),*)]
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Import {
	ValidationFunction(&'static str),
	ModificationFunction(&'static str),
	ValidyCore,
	ValidySettings,
	ValidyHelpers,
	AsyncTrait,
	TryFromMultipart,
	Deserialize,
	Serialize,
}

fn import_validation_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(crate::functions::validy::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::validation::#function_tokens)
		}
	}
}

fn import_modification_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(crate::functions::modification::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::modification::#function_tokens)
		}
	}
}

fn import_validy() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::core::*)
		}
	}
}

fn import_validy_settings() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::settings::*)
		}
	}
}

fn import_validy_helpers() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::utils::helpers::*)
		}
	}
}

fn import_async_trait() -> TokenStream {
	let found_crate = crate_name("async-trait").expect("async-trait is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::async_trait),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::async_trait)
		}
	}
}

fn import_try_from_multipart() -> TokenStream {
	let found_crate = crate_name("axum_typed_multipart").expect("axum_typed_multipart is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::TryFromMultipart),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::TryFromMultipart)
		}
	}
}

fn import_serialize() -> TokenStream {
	let found_crate = crate_name("serde").expect("serde is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::Serialize),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::Serialize)
		}
	}
}

fn import_deserialize() -> TokenStream {
	let found_crate = crate_name("serde").expect("serde is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::Deserialize),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::Deserialize)
		}
	}
}
