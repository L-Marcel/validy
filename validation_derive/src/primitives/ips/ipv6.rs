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

pub struct Ipv6Args {
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for Ipv6Args {
	fn default() -> Self {
		Ipv6Args {
			code: LitStr::new("ipv6", Span::call_site()),
			message: LitStr::new("invalid ipv6 format", Span::call_site()),
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

pub fn create_ipv6(input: ParseStream, field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ValidationFunction("ip::validate_ipv6 as validate_ipv6_fn"));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let Ipv6Args { code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => Ipv6Args::default(),
	};

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_ipv6_fn(#reference, #field_name, #code, #message) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_ipv6_fn(&#reference, #field_name, #code, #message) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
