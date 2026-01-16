use proc_macro_error::emit_error;
use syn::{DeriveInput, Error, Ident, LitBool, Result, Type, parse::ParseStream, spanned::Spanned};

use crate::primitives::commons::{ArgParser, parse_attrs};

#[derive(Default)]
pub struct ValidationAttributes {
	pub modify: bool,
	pub payload: bool,
	pub asynchronous: bool,
	pub context: Option<Type>,
	pub axum: bool,
}

impl ArgParser for ValidationAttributes {
	const POSITIONAL_KEYS: &'static [&'static str] = &["context", "modify", "payload", "asynchronous", "axum"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"context" => self.context = Some(input.parse()?),
			"asynchronous" => {
				let bool_lit: LitBool = input.parse()?;
				self.asynchronous = bool_lit.value();
			}
			"modify" => {
				let bool_lit: LitBool = input.parse()?;
				self.modify = bool_lit.value();
			}
			"payload" => {
				let bool_lit: LitBool = input.parse()?;
				self.payload = bool_lit.value();
			}
			"axum" => {
				let bool_lit: LitBool = input.parse()?;
				self.axum = bool_lit.value();
			}
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}

	fn apply_positional(&mut self, index: usize, input: ParseStream) -> Result<()> {
		if input.peek(Ident) {
			match input.parse::<Ident>()?.to_string().as_str() {
				"context" => self.context = Some(input.parse()?),
				"asynchronous" => self.asynchronous = true,
				"modify" => self.modify = true,
				"payload" => self.payload = true,
				"axum" => self.axum = true,
				_ => return Err(Error::new(input.span(), "unknown arg")),
			}

			Ok(())
		} else {
			let name = Self::POSITIONAL_KEYS
				.get(index)
				.ok_or_else(|| Error::new(input.span(), "too many positional args"))?;

			self.apply_value(name, input)
		}
	}
}

pub fn get_attributes(input: &DeriveInput) -> ValidationAttributes {
	let mut attributes = ValidationAttributes::default();

	for attr in &input.attrs {
		if attr.path().is_ident("validate") {
			let _ = attr.parse_args_with(|input: ParseStream| {
				attributes = parse_attrs(input)
					.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
					.unwrap_or_default();
				Ok(())
			});
		}
	}

	if attributes.axum && !cfg!(feature = "axum") {
		emit_error!(input.span(), "Needs to enable axum flag");
	}

	attributes
}
