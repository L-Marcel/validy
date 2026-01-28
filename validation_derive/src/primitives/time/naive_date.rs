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

pub struct NaiveDateArgs {
	pub format: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for NaiveDateArgs {
	fn default() -> Self {
		NaiveDateArgs {
			format: None,
			code: LitStr::new("naive_date", Span::call_site()),
			message: LitStr::new("invalid naive date format", Span::call_site()),
		}
	}
}

impl ArgParser for NaiveDateArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["format", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"format" => self.format = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_naive_date(
	input: ParseStream,
	field: &mut FieldAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	imports.borrow_mut().add(Import::ValidationFunction(
		"time::validate_naive_date as validate_naive_date_fn",
	));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let NaiveDateArgs { format, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => NaiveDateArgs::default(),
	};

	if format.is_none() {
		emit_error!(input.span(), "needs a format");
		return quote! {};
	}

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_naive_date_fn(#reference, #format, #field_name, #code, #message) {
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
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_naive_date_fn(_ref, #format, #field_name, #code, #message) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
