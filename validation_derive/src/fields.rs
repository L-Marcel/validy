use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, Index, LitStr, Type};

pub struct FieldAttributes {
	_type: Type,
	name: Option<Ident>,
	index: Option<Index>,
	scopes: usize,
	modifications: usize,
}

impl FieldAttributes {
	pub fn from_named(_type: &Type, name: &Ident) -> Self {
		FieldAttributes {
			_type: _type.clone(),
			name: Some(name.clone()),
			index: None,
			scopes: 0,
			modifications: 0,
		}
	}

	pub fn from_unamed(_type: &Type, index: &Index) -> Self {
		FieldAttributes {
			_type: _type.clone(),
			name: None,
			index: Some(index.clone()),
			scopes: 0,
			modifications: 0,
		}
	}

	pub fn get_type(&self) -> &Type {
		&self._type
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

		match (self.scopes, self.modifications) {
			(0, 0) => quote! { self.#suffix },
			(scopes, modifications) => {
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
