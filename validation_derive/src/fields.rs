use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, Index, LitStr, Type, parse_quote};

use crate::{attributes::ValidationAttributes, primitives::option::required::RequiredArgs};

pub struct FieldAttributes {
	ignore: bool,
	final_type: Type,
	current_type: Type,
	initial_type: Option<Type>,
	required_args: RequiredArgs,
	payload: bool,
	is_ref: bool,
	operations: Vec<TokenStream>,
	name: Option<Ident>,
	index: Option<Index>,
	scopes: usize,
	modifications: usize,
}

impl FieldAttributes {
	pub fn from_named(final_type: &Type, name: &Ident, attributes: &ValidationAttributes) -> Self {
		FieldAttributes {
			ignore: false,
			final_type: final_type.clone(),
			current_type: final_type.clone(),
			initial_type: None,
			required_args: RequiredArgs::default(),
			payload: attributes.payload,
			is_ref: false,
			operations: Vec::new(),
			name: Some(name.clone()),
			index: None,
			scopes: 0,
			modifications: 0,
		}
	}

	pub fn from_unamed(final_type: &Type, index: &Index, attributes: &ValidationAttributes) -> Self {
		FieldAttributes {
			ignore: false,
			final_type: final_type.clone(),
			current_type: final_type.clone(),
			initial_type: None,
			required_args: RequiredArgs::default(),
			payload: attributes.payload,
			is_ref: false,
			operations: Vec::new(),
			name: None,
			index: Some(index.clone()),
			scopes: 0,
			modifications: 0,
		}
	}

	pub fn add_operation(&mut self, operation: TokenStream) {
		self.operations.push(operation);
	}

	pub fn set_is_ref(&mut self, is_ref: bool) {
		self.is_ref = is_ref;
	}

	pub fn is_ref(&self) -> bool {
		let operations_is_empty = self.operations.iter().all(|operation| operation.is_empty());
		self.is_ref || (operations_is_empty && self.scopes == 0 && !self.is_payload() && self.is_option())
	}

	pub fn get_required_args(&self) -> RequiredArgs {
		self.required_args.clone()
	}

	pub fn get_operations(&mut self) -> Vec<TokenStream> {
		if self.ignore {
			Vec::new()
		} else {
			self.operations.clone()
		}
	}

	pub fn set_required_args(&mut self, required_args: RequiredArgs) {
		self.required_args = required_args;
	}

	pub fn is_payload(&self) -> bool {
		self.payload
	}

	pub fn is_option(&self) -> bool {
		if let Type::Path(type_path) = &self.final_type
			&& let Some(segment) = type_path.path.segments.last()
		{
			return segment.ident == "Option";
		}

		false
	}

	pub fn set_ignore(&mut self, ignore: bool) {
		self.ignore = ignore;
	}

	pub fn get_current_type(&self) -> &Type {
		&self.current_type
	}

	pub fn set_initial_type(&mut self, initial_type: &Type) {
		self.initial_type = Some(initial_type.clone());
	}

	pub fn set_current_type(&mut self, current_type: &Type) {
		self.current_type = current_type.clone();
		self.final_type = current_type.clone();
	}

	pub fn get_wrapper_final_type(&self) -> Type {
		let final_type = &self.final_type;
		if self.payload && !self.is_option() {
			let option_type: Type = parse_quote! {
				Option<#final_type>
			};

			option_type
		} else {
			let raw_type: Type = parse_quote! {
			  #final_type
			};

			raw_type
		}
	}

	pub fn get_initial_type(&self) -> Type {
		let initial_type = match &self.initial_type {
			Some(initial_type) => initial_type,
			None => &self.final_type,
		};

		if self.payload && (!self.is_option() || self.initial_type.is_some()) {
			let option_type: Type = parse_quote! {
				Option<#initial_type>
			};

			option_type
		} else {
			let raw_type: Type = parse_quote! {
			  #initial_type
			};

			raw_type
		}
	}

	pub fn get_name(&self) -> LitStr {
		match (&self.name, &self.index) {
			(Some(name), _) => LitStr::new(&name.to_string(), Span::call_site()),
			(_, Some(index)) => LitStr::new(&index.index.to_string(), Span::call_site()),
			_ => panic!("needs a field name or index"),
		}
	}

	pub fn increment_modifications(&mut self) {
		self.modifications += 1;
	}

	pub fn enter_scope(&mut self) {
		self.scopes += 1;
	}

	pub fn exit_scope(&mut self) {
		self.scopes -= 1;
	}

	pub fn get_wrapper_reference(&self) -> TokenStream {
		let suffix: &dyn ToTokens = match (&self.name, &self.index) {
			(Some(name), _) => name,
			(_, Some(index)) => index,
			_ => panic!("needs a field name or index"),
		};

		quote! { wrapper.#suffix }
	}

	pub fn get_original_reference(&self) -> TokenStream {
		let suffix: &dyn ToTokens = match (&self.name, &self.index) {
			(Some(name), _) => name,
			(_, Some(index)) => index,
			_ => panic!("needs a field name or index"),
		};

		quote! { self.#suffix }
	}

	pub fn get_unwrapped_reference(&self) -> Ident {
		let name = match (&self.name, &self.index) {
			(Some(name), _) => name.to_string(),
			(_, Some(index)) => index.index.to_string(),
			_ => panic!("needs a field name or index"),
		};

		let unwrapped_reference = format!("unwrapped_{}", name);
		Ident::new(&unwrapped_reference, Span::call_site())
	}

	pub fn get_reference(&self) -> TokenStream {
		let suffix: &dyn ToTokens = match (&self.name, &self.index) {
			(Some(name), _) => name,
			(_, Some(index)) => index,
			_ => panic!("needs a field name or index"),
		};

		match (self.payload || self.is_option(), self.scopes, self.modifications) {
			(false, 0, 0) => quote! { self.#suffix },
			(true, 0, 0) => {
				let name = match (&self.name, &self.index) {
					(Some(name), _) => name.to_string(),
					(_, Some(index)) => index.index.to_string(),
					_ => panic!("needs a field name or index"),
				};

				let final_name = format!("unwrapped_{}", name);
				let ident = Ident::new(&final_name, Span::call_site());
				quote! { #ident }
			}
			(_, scopes, modifications) => {
				let name = match (&self.name, &self.index) {
					(Some(name), _) => name.to_string(),
					(_, Some(index)) => index.index.to_string(),
					_ => panic!("needs a field name or index"),
				};

				let final_name = if scopes == 0 {
					format!("tmp_{}_{}", modifications, name)
				} else if modifications == 0 {
					format!("item_{}_{}", scopes, name)
				} else {
					format!("item_{}_tmp_{}_{}", scopes, modifications, name)
				};

				let ident = Ident::new(&final_name, Span::call_site());
				quote! { #ident }
			}
		}
	}
}
