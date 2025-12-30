use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Error, ExprRange, Ident, LitStr, Result, Token,
	parse::{ParseBuffer, ParseStream},
};

use crate::{imports::import_validation_functions, primitives::commons::remove_parens};

struct RangeArgs {
	range: Option<ExprRange>,
	code: LitStr,
	message: LitStr,
}

impl Default for RangeArgs {
	fn default() -> Self {
		RangeArgs {
			range: None,
			code: LitStr::new("range", Span::call_site()),
			message: LitStr::new("out of range", Span::call_site()),
		}
	}
}

pub fn create_range(field_name: &Option<Ident>, input: ParseStream) -> TokenStream {
	let content = remove_parens(input);
	let import = import_validation_functions("range::validate_range");

	let RangeArgs { range, code, message } = match content {
		Ok(content) => parse_range_attrs(content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => RangeArgs::default(),
	};

	if range.is_none() {
		emit_error!(input.span(), "need a range");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_range(#range, &self.#field_name, stringify!(#field_name), #code, #message) {
		  errors.push(e);
	  }
	}
}

fn parse_range_attrs(input: ParseBuffer<'_>) -> Result<RangeArgs> {
	let mut args = RangeArgs::default();
	let mut args_count = 0;
	let mut pos_index = 0;

	while !input.is_empty() {
		if input.peek(Ident) && input.peek2(Token![=]) {
			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;

			if args_count >= 3 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match key.to_string().as_str() {
				"range" => args.range = Some(input.parse()?),
				"code" => args.code = input.parse()?,
				"message" => args.message = input.parse()?,
				_ => return Err(Error::new(key.span(), "unknown arg")),
			}

			args_count += 1;
		} else {
			if args_count >= 3 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match pos_index {
				0 => args.range = Some(input.parse()?),
				1 => args.message = input.parse()?,
				2 => args.code = input.parse()?,
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
