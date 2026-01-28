use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, parse::ParseStream};

use crate::{
	attributes::ValidationAttributes,
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

#[derive(Default)]
pub struct IgnoreArgs {}

impl ArgParser for IgnoreArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &[];

	fn apply_value(&mut self, _: &str, input: ParseStream) -> Result<()> {
		Err(Error::new(input.span(), "unknown arg"))
	}
}

pub fn create_ignore(
	input: ParseStream,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	let content = remove_parens(input);

	let IgnoreArgs {} = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => IgnoreArgs::default(),
	};

	if !attributes.modificate {
		emit_error!(input.span(), "unnecessary for validation purposes only");
	}

	field.set_ignore(true);
	quote! {}
}
