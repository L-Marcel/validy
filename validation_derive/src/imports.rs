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

	pub fn create(&self) -> TokenStream {
		let imports: Vec<TokenStream> = self
			.set
			.iter()
			.map(|import| {
				let import = match import {
					Import::ValidationFunction(function) => import_validation_functions(function),
					Import::ModificationFunction(function) => import_modification_functions(function),
					Import::ParsingFunction(function) => import_parsing_functions(function),
					Import::ValidyCore => import_validy(),
					Import::ValidySettings => import_validy_settings(),
					Import::ValidyHelpers => import_validy_helpers(),
					Import::AsyncTrait => import_async_trait(),
				};

				quote! { use #import; }
			})
			.collect();

		quote! { #(#imports)* }
	}
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Import {
	ValidationFunction(&'static str),
	ModificationFunction(&'static str),
	ParsingFunction(&'static str),
	ValidyCore,
	ValidySettings,
	ValidyHelpers,
	AsyncTrait,
}

fn import_validation_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::functions::validation::#function_tokens),
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
		FoundCrate::Itself => quote!(::validy::functions::modification::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::modification::#function_tokens)
		}
	}
}

fn import_parsing_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::functions::parsing::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::parsing::#function_tokens)
		}
	}
}

fn import_validy() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::core::*)
		}
	}
}

fn import_validy_settings() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::settings::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::settings::*)
		}
	}
}

fn import_validy_helpers() -> TokenStream {
	let found_crate = crate_name("validy").expect("validy is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::utils::helpers::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::utils::helpers::*)
		}
	}
}

fn import_async_trait() -> TokenStream {
	let found_crate = crate_name("async-trait").expect("async-trait is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(::validy::async_trait),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::async_trait)
		}
	}
}
