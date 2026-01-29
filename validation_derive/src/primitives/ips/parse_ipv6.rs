use std::cell::RefCell;

use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	ImportsSet,
	fields::FieldAttributes,
	imports::Import,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct ParseIpArgs {
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for ParseIpArgs {
	fn default() -> Self {
		ParseIpArgs {
			code: LitStr::new("ipv6", Span::call_site()),
			message: LitStr::new("invalid ipv6 format", Span::call_site()),
		}
	}
}

impl ArgParser for ParseIpArgs {
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

pub fn create_ipv6_parse(
	input: ParseStream,
	field: &mut FieldAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ParsingFunction("ip::default_ipv6 as default_ipv6_fn"));
	imports
		.borrow_mut()
		.add(Import::ParsingFunction("ip::parse_ipv6 as parse_ipv6_fn"));

	let field_name = field.get_name();
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let content = remove_parens(input);

	let ParseIpArgs { code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => ParseIpArgs::default(),
	};

	if field.is_ref() {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
			let (mut #new_reference, error) = if can_continue(&errors, failure_mode, #field_name) {
			  parse_ipv6_fn(#reference, #field_name, #code, #message)
			} else {
			  (default_ipv6_fn(), None)
			};

			if let Some(e) = error {
				append_error(&mut errors, e, failure_mode, #field_name);
				if should_fail_fast(&errors, failure_mode, #field_name) {
				  return Err(errors);
			  };
		  }
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
  		let (mut #new_reference, error) = if can_continue(&errors, failure_mode, #field_name) {
        let _ref = &#reference;
  		  parse_ipv6_fn(_ref, #field_name, #code, #message)
  		} else {
  		  (default_ipv6_fn(), None)
  		};

			if let Some(e) = error {
				append_error(&mut errors, e, failure_mode, #field_name);
				if should_fail_fast(&errors, failure_mode, #field_name) {
				  return Err(errors);
			  };
		  }
		};

		result
	}
}
