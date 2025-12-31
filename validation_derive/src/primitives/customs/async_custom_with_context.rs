use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ExprArray, Ident, Result, parse::ParseStream};

use crate::{
	attributes::ValidationAttributes,
	fields::FieldAttributes,
	import_validation,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

#[derive(Default)]
pub struct AsyncCustomWithContextArgs {
	pub function: Option<Ident>,
	pub params: Option<ExprArray>,
}

impl ArgParser for AsyncCustomWithContextArgs {
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

pub fn create_async_custom_with_context(
	input: ParseStream,
	field: &FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	if !attributes.asynchronous {
		emit_error!(input.span(), "requires asynchronous attribute");
		return quote! {};
	}

	if attributes.context.is_none() {
		emit_error!(input.span(), "requires context attribute");
		return quote! {};
	}

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation();

	let AsyncCustomWithContextArgs { function, params } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => AsyncCustomWithContextArgs::default(),
	};

	if function.is_none() {
		let span = input.span();
		emit_error!(span, "needs a function");
		return quote! {};
	}

	let extra_args = params.iter().flat_map(|p| &p.elems).map(|arg| quote! { #arg });

	quote! {
	  use #import;
		if let Err(e) = #function(&#reference, #field_name, context, #(#extra_args),*).await {
		  errors.push(e);
		}
	}
}
