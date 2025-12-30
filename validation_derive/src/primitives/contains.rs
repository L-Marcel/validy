use proc_macro_error::{emit_error, emit_warning};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{LitStr, Token, parse::ParseStream};

use crate::{imports::import_validation_functions, primitives::commons::remove_parens};

pub fn create_contains(field_name: &Option<Ident>, input: ParseStream) -> Option<TokenStream> {
	let content = remove_parens(input);

	match content {
		Ok(content) => {
			let contains: Option<LitStr> = content.parse().ok();

			if let Some(content) = &contains {
				let span = content.span();

				emit_error!(
				  span, "need a value";
					help = "#[validate(contains(\"slug_\"))] pub regex: String"
				);

				return None;
			}

			let comma: Option<Token![,]> = content.parse().ok();
			let custom_message: Option<LitStr> = content.parse().ok();

			let message = match (content, comma, custom_message) {
				(content, Some(_), None) if !content.is_empty() => {
					let span = content.span();

					emit_warning!(
					  span, "expects a message after the comma";
						help = "#[validate(contains(\"slug_\", \"invlid format\"))] pub slug: String"
					);

					"invlid_format".to_string()
				}
				(content, Some(_), Some(message)) if !content.is_empty() => message.value(),
				(_, _, _) => "invlid format".to_string(),
			};

			let import = import_validation_functions("contains::validate_contains");
			Some(quote! {
			  use #import;
				if let Err(e) = validate_contains(&self.#field_name, #contains, stringify!(#field_name), #message) {
				  errors.push(e);
			  }
			})
		}
		Err(_) => {
			let span = input.span();

			emit_error!(
			  span, "need a value";
				help = "#[validate(contains(\"slug_\"))] pub regex: String"
			);

			None
		}
	}
}
