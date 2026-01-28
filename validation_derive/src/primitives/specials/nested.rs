use crate::primitives::commons::{ArgParser, parse_attrs, remove_parens};
use proc_macro_error::emit_error;
use proc_macro2::Span;
use syn::{Error, LitStr, Result, Type, parse::ParseStream};

pub struct NestedArgs {
	pub value: Option<Type>,
	pub wrapper: Option<Type>,
	pub code: LitStr,
}

impl Default for NestedArgs {
	fn default() -> Self {
		NestedArgs {
			value: None,
			wrapper: None,
			code: LitStr::new("nested", Span::call_site()),
		}
	}
}

impl ArgParser for NestedArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["value", "wrapper", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"value" => self.value = Some(input.parse()?),
			"wrapper" => self.wrapper = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn get_nested(input: ParseStream) -> (Option<Type>, Option<Type>, LitStr) {
	let content = remove_parens(input);
	let NestedArgs { value, wrapper, code } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => NestedArgs::default(),
	};

	if let Some(nested_type) = &value {
		(Some(nested_type.clone()), wrapper, code)
	} else {
		emit_error!(input.span(), "needs the value type");

		(None, None, code)
	}
}
