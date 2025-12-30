use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Error, Ident, LitStr, Result, Token,
	parse::{ParseBuffer, ParseStream},
};

use crate::{imports::import_validation_functions, primitives::commons::remove_parens};

struct EmailArgs {
	code: LitStr,
	message: LitStr,
}

impl Default for EmailArgs {
	fn default() -> Self {
		EmailArgs {
			code: LitStr::new("email", Span::call_site()),
			message: LitStr::new("invalid email format", Span::call_site()),
		}
	}
}

pub fn create_email(field_name: &Option<Ident>, input: ParseStream) -> Option<TokenStream> {
	let content = remove_parens(input);
	let import = import_validation_functions("email::validate_email");

	let EmailArgs { code, message } = match content {
		Ok(content) => parse_email_attrs(content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => EmailArgs::default(),
	};

	Some(quote! {
	  use #import;
		if let Err(e) = validate_email(&self.#field_name, stringify!(#field_name), #code, #message) {
		  errors.push(e);
	  }
	})
}

fn parse_email_attrs(input: ParseBuffer<'_>) -> Result<EmailArgs> {
	let mut args = EmailArgs::default();
	let mut args_count = 0;
	let mut pos_index = 0;

	while !input.is_empty() {
		if input.peek(Ident) && input.peek2(Token![=]) {
			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;

			if args_count >= 2 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match key.to_string().as_str() {
				"code" => args.code = input.parse()?,
				"message" => args.message = input.parse()?,
				_ => return Err(Error::new(key.span(), "unknown arg")),
			}

			args_count += 1;
		} else {
			if args_count >= 2 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match pos_index {
				0 => args.message = input.parse()?,
				1 => args.code = input.parse()?,
				_ => return Err(Error::new(input.span(), "too many positional args")),
			}

			pos_index += 1;
			args_count += 1;
		}

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(args)
}
