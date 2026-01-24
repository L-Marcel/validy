use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ExprArray, Ident, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

#[derive(Default)]
struct CustomArgs {
	function: Option<Ident>,
	params: Option<ExprArray>,
}

impl ArgParser for CustomArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["function", "params"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"function" => self.function = Some(input.parse()?),
			"params" => self.params = Some(input.parse()?),
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_custom(input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let CustomArgs { function, params } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => CustomArgs::default(),
	};

	if function.is_none() {
		let span = input.span();
		emit_error!(span, "needs a function");
		return quote! {};
	}

	let extra_args = params.iter().flat_map(|p| &p.elems).map(|arg| quote! { #arg });

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = #function(#reference, #field_name, #(#extra_args),*) {
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
			if can_continue(&errors, failure_mode, #field_name) && let Err(e) = #function(&#reference, #field_name, #(#extra_args),*) {
        append_error(&mut errors, e, failure_mode, #field_name);
        if should_fail_fast(&errors, failure_mode, #field_name) {
     			return Err(errors);
     	  }
			}
		};

		result
	}
}
