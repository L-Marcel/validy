use crate::primitives::commons::{ArgParser, parse_attrs};
use proc_macro_error::emit_error;
use syn::{DeriveInput, Error, Expr, Ident, LitBool, Result, Type, parse::ParseStream, spanned::Spanned};

#[derive(Default)]
pub struct ValidationAttributes {
	pub modificate: bool,
	pub payload: bool,
	pub asynchronous: bool,
	pub context: Option<Type>,
	pub axum: bool,
	pub multipart: bool,
	pub failure_mode: Option<Expr>,
}

impl ArgParser for ValidationAttributes {
	const POSITIONAL_KEYS: &'static [&'static str] = &[
		"context",
		"modificate",
		"payload",
		"asynchronous",
		"axum",
		"multipart",
		"failure_mode",
	];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"context" => self.context = Some(input.parse()?),
			"failure_mode" => {
				let failure_mode: Expr = input.parse()?;
				validate_failure_mode(&input, &failure_mode);
				self.failure_mode = Some(failure_mode);
			}
			"asynchronous" => {
				let bool_lit: LitBool = input.parse()?;
				self.asynchronous = bool_lit.value();
			}
			"modificate" => {
				let bool_lit: LitBool = input.parse()?;
				self.modificate = bool_lit.value();
			}
			"payload" => {
				let bool_lit: LitBool = input.parse()?;
				self.payload = bool_lit.value();
			}
			"axum" => {
				let bool_lit: LitBool = input.parse()?;
				self.axum = bool_lit.value();
			}
			"multipart" => {
				let bool_lit: LitBool = input.parse()?;
				self.multipart = bool_lit.value();
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
				"modificate" => self.modificate = true,
				"payload" => self.payload = true,
				"axum" => self.axum = true,
				"multipart" => self.multipart = true,
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

	match (
		attributes.axum,
		attributes.multipart,
		cfg!(feature = "axum"),
		cfg!(feature = "axum_multipart"),
	) {
		(true, _, false, _) => emit_error!(input.span(), "Needs to enable axum flag"),
		(true, true, true, false) => emit_error!(input.span(), "Needs to enable axum_multipart flag"),
		(false, true, true, true) => emit_error!(input.span(), "Needs to enable axum configuration attribute"),
		_ => {}
	}

	attributes
}

pub fn validate_failure_mode(input: &ParseStream, expr: &Expr) {
	let path = match expr {
		Expr::Path(expr_path) => &expr_path.path,
		_ => {
			emit_error!(input.span(), "Expects a FailureMode enum variant");
			return;
		}
	};

	let segments: Vec<_> = path.segments.iter().collect();

	if segments.is_empty() {
		emit_error!(input.span(), "Expects a FailureMode enum variant");
		return;
	}

	let variant_ident = &segments.last().unwrap().ident;
	let variant_str = variant_ident.to_string();

	let valid_variants = ["FailFast", "FailOncePerField", "LastFailPerField", "FullFail"];
	if !valid_variants.contains(&variant_str.as_str()) {
		emit_error!(input.span(), "Unknown FailureMode enum variant");
		return;
	};

	if segments.len() > 1 {
		emit_error!(input.span(), "Too big path");
	};
}
