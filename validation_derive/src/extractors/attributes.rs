use proc_macro_error::emit_error;
use syn::{
	DeriveInput, Error, Ident, LitBool, Result, Token, Type,
	parse::{ParseBuffer, ParseStream},
};

#[derive(Default)]
pub struct ValidationAttributes {
	pub modify: bool,
	pub payload: bool,
	pub asynchronous: bool,
	pub context: Option<Type>,
}

pub fn get_attributes(input: &DeriveInput) -> ValidationAttributes {
	let mut attributes = ValidationAttributes::default();

	for attr in &input.attrs {
		if attr.path().is_ident("validate") {
			let result = attr.parse_args_with(|input: ParseStream| parse_validation_attrs(input, &mut attributes));

			if let Err(err) = result {
				emit_error!(err.span(), "{}", err);
			}
		}
	}

	attributes
}

fn parse_validation_attrs(input: &ParseBuffer<'_>, args: &mut ValidationAttributes) -> Result<()> {
	let mut args_count = 0;
	let mut pos_index = 0;

	while !input.is_empty() {
		if input.peek(Ident) && input.peek2(Token![=]) {
			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;

			if args_count >= 2 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match key.to_string().as_str() {
				"context" => {
					let ty: Type = input.parse()?;
					args.context = Some(ty);
				}
				"asynchronous" => {
					let bool_lit: LitBool = input.parse()?;
					args.asynchronous = bool_lit.value();
				}
				"modify" => {
					let bool_lit: LitBool = input.parse()?;
					args.modify = bool_lit.value();
				}
				"payload" => {
					let bool_lit: LitBool = input.parse()?;
					args.payload = bool_lit.value();
				}
				_ => return Err(Error::new(key.span(), "unknown arg")),
			}

			args_count += 1;
		} else {
			if args_count >= 2 {
				return Err(Error::new(input.span(), "too many args"));
			}

			match pos_index {
				0 => {
					let ty: Type = input.parse()?;
					args.context = Some(ty);
				}
				1 => {
					let bool_lit: LitBool = input.parse()?;
					args.asynchronous = bool_lit.value();
				}
				2 => {
					let bool_lit: LitBool = input.parse()?;
					args.modify = bool_lit.value();
				}
				3 => {
					let bool_lit: LitBool = input.parse()?;
					args.payload = bool_lit.value();
				}
				_ => return Err(Error::new(input.span(), "too many positional args")),
			}

			pos_index += 1;
			args_count += 1;
		}

		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(())
}
