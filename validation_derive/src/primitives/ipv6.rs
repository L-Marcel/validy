use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{LitStr, parse::ParseStream};

use crate::{imports::import_validation_functions, primitives::commons::remove_parens};

pub fn create_ipv6(field_name: &Option<Ident>, input: ParseStream) -> Option<TokenStream> {
	let content = remove_parens(input);

	match content {
		Ok(content) => {
			let custom_message: Option<LitStr> = content.parse().ok();

			let message = match (content, custom_message) {
				(content, Some(message)) if !content.is_empty() => message.value(),
				(_, _) => "invalid ipv6 format".to_string(),
			};

			let import = import_validation_functions("ip::validate_ipv6");
			Some(quote! {
			  use #import;
				if let Err(e) = validate_ipv6(&self.#field_name, stringify!(#field_name), #message) {
				  errors.push(e);
			  }
			})
		}
		Err(_) => {
			let import = import_validation_functions("ip::validate_ipv6");
			Some(quote! {
			  use #import;
				if let Err(e) = validate_ipv6(&self.#field_name, stringify!(#field_name), "invalid ipv6 format") {
				  errors.push(e);
			  }
			})
		}
	}
}
