use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	Error, ExprArray, Ident, Result, Token,
	parse::{ParseBuffer, ParseStream},
};

use crate::{import_validation, primitives::commons::remove_parens};

#[derive(Default)]
struct CustomArgs {
	function: Option<Ident>,
	params: Option<ExprArray>,
}

pub fn create_custom(field_name: &Option<Ident>, input: ParseStream) -> TokenStream {
	let content = remove_parens(input);
	let import = import_validation();

	let CustomArgs { function, params } = match content {
		Ok(content) => parse_custom_attrs(content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => CustomArgs::default(),
	};

	let extra_args = params.iter().flat_map(|p| &p.elems).map(|arg| quote! { #arg });

	quote! {
	  use #import;
		if let Err(e) = #function(&self.#field_name, #(#extra_args),*) {
		  errors.push(e);
		}
	}
}

fn parse_custom_attrs(input: ParseBuffer<'_>) -> Result<CustomArgs> {
	let mut args = CustomArgs::default();
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
				"function" => args.function = input.parse()?,
				"params" => args.params = Some(input.parse()?),
				_ => return Err(Error::new(key.span(), "unknown arg")),
			}

			args_count += 1;
		} else {
			if args_count >= 2 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match pos_index {
				0 => args.function = input.parse()?,
				1 => args.params = Some(input.parse()?),
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
