use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Path, parse_str};

pub fn import_validation_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validation").expect("validation is present in `Cargo.toml`");
	let function_path: Path = parse_str(function).expect("can't parse validation crate path");

	match found_crate {
		FoundCrate::Itself => quote!(crate::functions::#function),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::#function_path)
		}
	}
}

pub fn import_validation() -> TokenStream {
	let found_crate = crate_name("validation").expect("validation is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::core::*)
		}
	}
}

pub fn import_async_trait() -> TokenStream {
	let found_crate = crate_name("async-trait").expect("async-trait is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::async_trait),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::async_trait)
		}
	}
}
