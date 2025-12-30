use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{LitStr, parse::ParseStream};

use crate::{imports::import_validation_functions, primitives::commons::remove_parens};

pub fn create_ip(field_name: &Option<Ident>, input: ParseStream) -> Option<TokenStream> {
	let content = remove_parens(input);

	match content {
		Ok(content) => {
			let custom_message: Option<LitStr> = content.parse().ok();

			let message = match (content, custom_message) {
				(content, Some(message)) if !content.is_empty() => message.value(),
				(_, _) => "invalid ip format".to_string(),
			};

			let import = import_validation_functions("ip::validate_ip");
			Some(quote! {
			  use #import;
				if let Err(e) = validate_ip(&self.#field_name, stringify!(#field_name), #message) {
				  errors.push(e);
			  }
			})
		}
		Err(_) => {
			let import = import_validation_functions("ip::validate_ip");
			Some(quote! {
			  use #import;
				if let Err(e) = validate_ip(&self.#field_name, stringify!(#field_name), "invalid ip format") {
				  errors.push(e);
			  }
			})
		}
	}
}
