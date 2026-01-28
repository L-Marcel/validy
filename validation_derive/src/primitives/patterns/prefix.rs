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

pub struct PrefixArgs {
	pub prefix: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for PrefixArgs {
	fn default() -> Self {
		PrefixArgs {
			prefix: None,
			code: LitStr::new("prefix", Span::call_site()),
			message: LitStr::new("invalid prefix", Span::call_site()),
		}
	}
}

impl ArgParser for PrefixArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["prefix", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"prefix" => self.prefix = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_prefix(input: ParseStream, field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ValidationFunction(
		"prefix::validate_prefix as validate_prefix_fn",
	));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let PrefixArgs { prefix, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => PrefixArgs::default(),
	};

	if prefix.is_none() {
		let span = input.span();
		emit_error!(span, "needs a prefix");
		return quote! {};
	}

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_prefix_fn(#reference, #prefix, #field_name, #code, #message) {
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
		  let _ref = &#reference;
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_prefix_fn(_ref, #prefix, #field_name, #code, #message) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
