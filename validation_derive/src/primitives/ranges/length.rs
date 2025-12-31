use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprRange, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct LengthArgs {
	pub range: Option<ExprRange>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for LengthArgs {
	fn default() -> Self {
		LengthArgs {
			range: None,
			code: LitStr::new("length", Span::call_site()),
			message: LitStr::new("length out of range", Span::call_site()),
		}
	}
}

impl ArgParser for LengthArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["range", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"range" => self.range = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_length(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("length::validate_length");

	let LengthArgs { range, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => LengthArgs::default(),
	};

	if range.is_none() {
		emit_error!(input.span(), "needs a range");
	}

	quote! {
	  use #import;
		if let Err(e) = validate_length(&#reference, #range, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
