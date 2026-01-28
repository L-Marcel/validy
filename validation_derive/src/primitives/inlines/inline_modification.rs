use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ExprArray, ExprClosure, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

#[derive(Default)]
pub struct InlineModificationArgs {
	pub closure: Option<ExprClosure>,
	pub params: Option<ExprArray>,
}

impl ArgParser for InlineModificationArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["closure", "params"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"closure" => self.closure = Some(input.parse()?),
			"params" => self.params = Some(input.parse()?),
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_inline_modification(input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	let content = remove_parens(input);

	let InlineModificationArgs { closure, params } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => InlineModificationArgs::default(),
	};

	if closure.is_none() {
		emit_error!(input.span(), "needs a closure");
		return quote! {};
	}

	let extra_args = params.iter().flat_map(|p| &p.elems).map(|arg| quote! { #arg });
	let field_name = field.get_name();

	if field.is_ref() {
		field.set_is_ref(true);
		#[rustfmt::skip]
		let result = quote! {
			if can_continue(&errors, failure_mode, #field_name) {
			  (#closure)(#reference, #(#extra_args),*);
      };
		};

		result
	} else {
		field.set_is_ref(false);
		#[rustfmt::skip]
		let result = quote! {
      if can_continue(&errors, failure_mode, #field_name) {
        let _ref = &mut #reference;
        (#closure)(_ref, #(#extra_args),*);
      };
		};

		result
	}
}
