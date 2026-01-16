use std::cell::RefCell;

use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprArray, LitStr, Result, parse::ParseStream};

use crate::{
	ImportsSet,
	fields::FieldAttributes,
	imports::Import,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct BlocklistArgs {
	pub mode: Option<LitStr>,
	pub items: Option<ExprArray>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for BlocklistArgs {
	fn default() -> Self {
		BlocklistArgs {
			mode: None,
			items: None,
			code: LitStr::new("blocklist", Span::call_site()),
			message: LitStr::new("has item inside blocklist", Span::call_site()),
		}
	}
}

impl ArgParser for BlocklistArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["mode", "items", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"mode" => self.mode = Some(input.parse()?),
			"items" => self.items = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_blocklist(input: ParseStream, field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ValidationFunction(
		"iter::validate_blocklist as validate_blocklist_fn",
	));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let BlocklistArgs {
		mode,
		items,
		code,
		message,
	} = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => BlocklistArgs::default(),
	};

	if items.is_none() {
		emit_error!(input.span(), "needs a collection of items to use as blocklist");
		return quote! {};
	}

	if field.is_ref() {
		field.set_is_ref(true);
	} else {
		field.set_is_ref(false);
	};

	match mode {
		Some(mode) if mode.value() == "SINGLE" => {
			quote! {
				if let Err(e) = validate_blocklist_fn([#reference.clone()], #items, #field_name, #code, #message) {
				  errors.push(e);
			  }
			}
		}
		Some(mode) if mode.value() == "COLLECTION" => {
			quote! {
				if let Err(e) = validate_blocklist_fn(#reference.clone(), #items, #field_name, #code, #message) {
				  errors.push(e);
			  }
			}
		}
		Some(_) => {
			emit_error!(input.span(), "available modes are SINGLE and COLLECTION");
			quote! {}
		}
		None => {
			emit_error!(input.span(), "needs the mode");
			quote! {}
		}
	}
}
