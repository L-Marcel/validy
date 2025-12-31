use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct ContainsArgs {
	pub items: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for ContainsArgs {
	fn default() -> Self {
		ContainsArgs {
			items: None,
			code: LitStr::new("items", Span::call_site()),
			message: LitStr::new("invalid format", Span::call_site()),
		}
	}
}

impl ArgParser for ContainsArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["items", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"items" => self.items = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_contains(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("contains::validate_contains");

	let ContainsArgs { items, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => ContainsArgs::default(),
	};

	if items.is_none() {
		emit_error!(input.span(), "needs a slice of string to check");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_contains(&#reference, #items, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
