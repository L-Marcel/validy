use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, Index, LitStr, Type, parse_quote};

use crate::primitives::option::required::RequiredArgs;

pub struct FieldAttributes {
	_type: Type,
	_initial_type: Option<Type>,
	required_args: RequiredArgs,
	as_payload: bool,
	operations: Vec<TokenStream>,
	name: Option<Ident>,
	index: Option<Index>,
	scopes: usize,
	modifications: usize,
}

impl FieldAttributes {
	pub fn from_named(_type: &Type, name: &Ident, as_payload: bool) -> Self {
		FieldAttributes {
			_type: _type.clone(),
			_initial_type: None,
			required_args: RequiredArgs::default(),
			as_payload,
			operations: Vec::new(),
			name: Some(name.clone()),
			index: None,
			scopes: 0,
			modifications: 0,
		}
	}

	pub fn from_unamed(_type: &Type, index: &Index, as_payload: bool) -> Self {
		FieldAttributes {
			_type: _type.clone(),
			_initial_type: None,
			required_args: RequiredArgs::default(),
			as_payload,
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

	pub fn get_operations(&mut self) -> TokenStream {
		if self.as_payload {
			let field_name = &self.get_name();
			let field_type = &self.get_final_type();
			let reference = &self.get_reference();
			self.increment_modifications();
			let new_reference = &self.get_reference();
			let wrapper_reference = &self.get_wrapper_reference();
			let operations = &self.operations;

			let name = match (&self.name, &self.index) {
				(Some(name), _) => name.to_string(),
				(_, Some(index)) => index.index.to_string(),
				_ => panic!("needs a field name or index"),
			};

			let unwrapped_final_name = format!("unwrapped_{}", name);
			let unwrapped = Ident::new(&unwrapped_final_name, Span::call_site());

			if self.is_option() {
				quote! {
					let mut #new_reference: #field_type = None;
					if let Some(#unwrapped) = #wrapper_reference.as_ref() {
						#(#operations)*
						#new_reference = Some(#reference);
					}
				}
			} else {
				let code = &self.required_args.code;
				let message = &self.required_args.message;

				quote! {
				  let mut #new_reference: #field_type = None;
				  if let Some(#unwrapped) = #wrapper_reference.as_ref() {
						#(#operations)*
						#new_reference = Some(#reference);
					} else {
					  errors.push(ValidationError::builder()
								.with_field(#field_name)
								.as_simple(#code)
								.with_message(#message)
								.build()
								.into());
					}
				}
			}
		} else {
			let operations = &self.operations;
			quote! {
			  #(#operations)*
			}
		}
	}

	pub fn set_required_args(&mut self, required_args: RequiredArgs) {
		self.required_args = required_args;
	}

	pub fn is_option(&self) -> bool {
		if let Type::Path(type_path) = &self._type
			&& let Some(segment) = type_path.path.segments.last()
		{
			return segment.ident == "Option";
		}

		false
	}

	pub fn get_type(&self) -> &Type {
		&self._type
	}

	pub fn get_final_type(&self) -> Type {
		let _type = &self._type;
		if self.as_payload && !self.is_option() {
			let _option_type: Type = parse_quote! {
				Option<#_type>
			};

			_option_type
		} else {
			let __type: Type = parse_quote! {
			  #_type
			};

			__type
		}
	}

	pub fn get_initial_type(&self) -> Type {
		let _type = match &self._initial_type {
			Some(_type) => _type,
			None => &self._type,
		};

		if self.as_payload && !self.is_option() {
			let _option_type: Type = parse_quote! {
				Option<#_type>
			};

			_option_type
		} else {
			let __type: Type = parse_quote! {
			  #_type
			};

			__type
		}
	}

	pub fn get_name(&self) -> LitStr {
		match (&self.name, &self.index) {
			(Some(name), _) => LitStr::new(&name.to_string(), Span::call_site()),
			(_, Some(index)) => LitStr::new(&index.index.to_string(), Span::call_site()),
			_ => panic!("needs a field name or index"),
		}
	}

	pub fn get_modifications(&self) -> usize {
		self.modifications
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

		quote! { ___wrapper.#suffix }
	}

	pub fn get_original_reference(&self) -> TokenStream {
		let suffix: &dyn ToTokens = match (&self.name, &self.index) {
			(Some(name), _) => name,
			(_, Some(index)) => index,
			_ => panic!("needs a field name or index"),
		};

		quote! { self.#suffix }
	}

	pub fn get_reference(&self) -> TokenStream {
		let suffix: &dyn ToTokens = match (&self.name, &self.index) {
			(Some(name), _) => name,
			(_, Some(index)) => index,
			_ => panic!("needs a field name or index"),
		};

		match (self.as_payload, self.scopes, self.modifications) {
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
