use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct Ipv6Args {
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for Ipv6Args {
	fn default() -> Self {
		Ipv6Args {
			code: LitStr::new("ipv4", Span::call_site()),
			message: LitStr::new("invalid ipv4 format", Span::call_site()),
		}
	}
}

impl ArgParser for Ipv6Args {
	const POSITIONAL_KEYS: &'static [&'static str] = &["message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_ipv6(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("ip::validate_ipv6");

	let Ipv6Args { code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => Ipv6Args::default(),
	};

	quote! {
	  use #import;
		if let Err(e) = validate_ipv6(&#reference, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
