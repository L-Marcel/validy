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

pub struct SuffixArgs {
	pub suffix: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for SuffixArgs {
	fn default() -> Self {
		SuffixArgs {
			suffix: None,
			code: LitStr::new("suffix", Span::call_site()),
			message: LitStr::new("invalid suffix", Span::call_site()),
		}
	}
}

impl ArgParser for SuffixArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["suffix", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"suffix" => self.suffix = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_suffix(input: ParseStream, field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ValidationFunction(
		"suffix::validate_suffix as validate_suffix_fn",
	));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let SuffixArgs { suffix, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => SuffixArgs::default(),
	};

	if suffix.is_none() {
		let span = input.span();
		emit_error!(span, "needs a suffix");
		return quote! {};
	}

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_suffix_fn(#reference, #suffix, #field_name, #code, #message) {
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
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = validate_suffix_fn(&#reference, #suffix, #field_name, #code, #message) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
