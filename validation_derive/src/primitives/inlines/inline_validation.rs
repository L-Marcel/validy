use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprArray, ExprClosure, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct InlineValidationArgs {
	pub closure: Option<ExprClosure>,
	pub params: Option<ExprArray>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for InlineValidationArgs {
	fn default() -> Self {
		InlineValidationArgs {
			closure: None,
			params: None,
			code: LitStr::new("inline", Span::call_site()),
			message: LitStr::new("invalid", Span::call_site()),
		}
	}
}

impl ArgParser for InlineValidationArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["closure", "params", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"closure" => self.closure = Some(input.parse()?),
			"params" => self.params = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_inline_validation(input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	let field_name = field.get_name();
	let content = remove_parens(input);

	let InlineValidationArgs {
		closure,
		params,
		code,
		message,
	} = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => InlineValidationArgs::default(),
	};

	if closure.is_none() {
		emit_error!(input.span(), "needs a closure");
		return quote! {};
	}

	let extra_args = params.iter().flat_map(|p| &p.elems).map(|arg| quote! { #arg });

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
		  if can_continue(&errors, failure_mode, #field_name) && !(#closure)(#reference, #(#extra_args),*) {
				let error = ValidationError::builder()
  			  .with_field(#field_name)
  			  .as_simple(#code)
  			  .with_message(#message)
  			  .build();

        append_error(&mut errors, error.into(), failure_mode, #field_name);
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
		  if can_continue(&errors, failure_mode, #field_name) && !(#closure)(&#reference, #(#extra_args),*) {
				let error = ValidationError::builder()
  			  .with_field(#field_name)
  			  .as_simple(#code)
  			  .with_message(#message)
  			  .build();

        append_error(&mut errors, error.into(), failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
		  }
		};

		result
	}
}
