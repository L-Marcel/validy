use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct PatternArgs {
	pub pattern: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for PatternArgs {
	fn default() -> Self {
		PatternArgs {
			pattern: None,
			code: LitStr::new("pattern", Span::call_site()),
			message: LitStr::new("outside the accepted pattern", Span::call_site()),
		}
	}
}

impl ArgParser for PatternArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["pattern", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"pattern" => self.pattern = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_pattern(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("pattern::validate_pattern");

	let PatternArgs { pattern, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => PatternArgs::default(),
	};

	if let Some(content) = &pattern {
		let regex = Regex::new(&content.value());

		if let Err(err) = regex {
			let span = content.span();

			emit_error!(
			  span, "invalid pattern";
				help = err
			);

			return quote! {};
		}
	} else {
		let span = input.span();
		emit_error!(span, "needs a pattern");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_pattern(&#reference, #pattern, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
