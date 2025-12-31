use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprRange, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct RangeArgs {
	pub range: Option<ExprRange>,
	pub code: LitStr,
	pub message: LitStr,
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

impl ArgParser for RangeArgs {
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

pub fn create_range(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("range::validate_range");

	let RangeArgs { range, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => RangeArgs::default(),
	};

	if range.is_none() {
		emit_error!(input.span(), "needs a range");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_range(&#reference, #range, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
