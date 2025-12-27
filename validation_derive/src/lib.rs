use core::panic;
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Data, Ident, LitStr, Path, Type, meta::ParseNestedMeta};

fn import_validation() -> proc_macro2::TokenStream {
	let found_crate = crate_name("validation").expect("validation is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::core::*)
		}
	}
}

fn import_async_trait() -> proc_macro2::TokenStream {
	let found_crate = crate_name("async-trait").expect("async-trait is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::async_trait),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::async_trait)
		}
	}
}

#[proc_macro_derive(Validate)]
pub fn validation_macro(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_validation_macro(&ast, false)
}

#[proc_macro_derive(AsyncValidate)]
pub fn async_validation_macro(input: TokenStream) -> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_validation_macro(&ast, true)
}

fn impl_validation_macro(ast: &syn::DeriveInput, asynchronous: bool) -> TokenStream {
	let name = &ast.ident;

	let fields = if let Data::Struct(data) = &ast.data {
		&data.fields
	} else {
		panic!("validation only supports structs!");
	};

	let field_validations: Vec<(Vec<ValidationQuote>, bool)> = fields
		.iter()
		.map(|field| {
			let field_name = &field.ident;
			let field_type = &field.ty;

			let mut with_context = false;
			let mut validations = Vec::<ValidationQuote>::new();
			for attr in &field.attrs {
				if attr.path().is_ident("validate") {
					let _ = attr.parse_nested_meta(|meta| {
						let (validation, has_context) =
							apply_validation_attr_macro(field_name, field_type, meta, asynchronous);

						with_context |= has_context;
						validations.push(validation);
						Ok(())
					});
				}
			}

			(validations, with_context)
		})
		.collect();

	let need_context = field_validations
		.iter()
		.map(|(_, with_context)| *with_context)
		.reduce(|a, b| a || b)
		.unwrap_or(false);

	let field_checks = field_validations.iter().map(|(field_validations, _)| {
		let validations: Vec<proc_macro2::TokenStream> = field_validations
			.iter()
			.map(|validation| match validation {
				ValidationQuote::Simple(simple) => simple.clone(),
				ValidationQuote::Conditional(simple, complex) => {
					if need_context {
						complex.clone()
					} else {
						simple.clone()
					}
				}
			})
			.collect();

		quote! { #(#validations)* }
	});

	let (strait, with_context_strait, func_name, with_context_func_name, _) = get_idents(asynchronous);

	let async_kw = if asynchronous { quote!(async) } else { quote!() };
	let async_import = if asynchronous { import_async_trait() } else { quote!() };
	let async_attr = if asynchronous {
		quote!(#[::async_trait::async_trait])
	} else {
		quote!()
	};

	let import = import_validation();
	if need_context {
		quote! {
		  use #import;
		  use #async_import;

		  #async_attr
		  impl<C> #with_context_strait<C> for #name {
			  #async_kw fn #with_context_func_name(&self, context: &C) -> Result<(), ValidationErrors> {
				  let mut errors = Vec::<ValidationError>::new();
				  #(#field_checks)*

				  if errors.is_empty() {
					  Ok(())
				  } else {
					  let map = errors.into_iter()
						  .map(|e| (e.field.clone(), e))
						  .collect();

					  Err(map)
				  }
			  }
		  }
		}
		.into()
	} else {
		quote! {
		  use #import;
		  use #async_import;

			#async_attr
		  impl #strait for #name {
			  #async_kw fn #func_name(&self) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();
				  #(#field_checks)*

				  if errors.is_empty() {
					  Ok(())
				  } else {
					  let map = errors.into_iter()
						  .map(|e| (e.field.clone(), e))
						  .collect();

					  Err(map)
				  }
			  }
		  }
		}
		.into()
	}
}

enum ValidationQuote {
	Simple(proc_macro2::TokenStream),
	Conditional(proc_macro2::TokenStream, proc_macro2::TokenStream),
}

fn get_idents(asynchronous: bool) -> (Ident, Ident, Ident, Ident, proc_macro2::TokenStream) {
	let strait = format_ident!("{}", if asynchronous { "AsyncValidation" } else { "Validation" });
	let with_context_strait = format_ident!(
		"{}",
		if asynchronous {
			"AsyncValidationWithContext"
		} else {
			"ValidationWithContext"
		}
	);

	let func_name = format_ident!("{}", if asynchronous { "async_validate" } else { "validate" });
	let with_context_func_name = format_ident!(
		"{}",
		if asynchronous {
			"async_validate_with_context"
		} else {
			"validate_with_context"
		}
	);

	let suffix = if asynchronous { quote!(.await) } else { quote!() };

	(strait, with_context_strait, func_name, with_context_func_name, suffix)
}

fn apply_validation_attr_macro(
	field_name: &Option<Ident>,
	field_type: &Type,
	meta: ParseNestedMeta<'_>,
	asynchronous: bool,
) -> (ValidationQuote, bool) {
	let (strait, with_context_strait, func_name, with_context_func_name, suffix) = get_idents(asynchronous);

	match meta {
		m if m.path.is_ident("nested") => (
			ValidationQuote::Conditional(
				quote! {
				  if let Err(e) = <#field_type as #strait>::#func_name(&self.#field_name)#suffix {
						todo!()
				  }
				},
				quote! {
				  if let Err(e) = <#field_type as #with_context_strait<C>>::#with_context_func_name(&self.#field_name, &context)#suffix {
						todo!()
					}
				},
			),
			false,
		),
		m if m.path.is_ident("custom") => {
			let val: LitStr = m
				.value()
				.unwrap_or_else(|_| panic!("'custom' need a value"))
				.parse()
				.unwrap_or_else(|_| panic!("'custom' value should be a function name"));

			let custom_func_name: Path = val
				.parse()
				.unwrap_or_else(|_| panic!("'custom' value should be a function name"));

			(
				ValidationQuote::Simple(quote! {
					if let Err(e) = #custom_func_name(&self.#field_name) {
					  errors.push(e);
					}
				}),
				false,
			)
		}
		m if m.path.is_ident("async_custom") => {
			let val: LitStr = m
				.value()
				.unwrap_or_else(|_| panic!("'async_custom' need a value"))
				.parse()
				.unwrap_or_else(|_| panic!("'async_custom' value should be a function name"));

			let custom_func_name: Path = val
				.parse()
				.unwrap_or_else(|_| panic!("'async_custom' value should be a function name"));

			(
				ValidationQuote::Simple(quote! {
					if let Err(e) = #custom_func_name(&self.#field_name).await {
					  errors.push(e);
					}
				}),
				false,
			)
		}
		m if m.path.is_ident("custom_with_context") => {
			let val: LitStr = m
				.value()
				.unwrap_or_else(|_| panic!("'custom_with_context' need a value"))
				.parse()
				.unwrap_or_else(|_| panic!("'custom_with_context' value should be a function name"));

			let custom_func_name: Path = val
				.parse()
				.unwrap_or_else(|_| panic!("'custom_with_context' value should be a function name"));

			(
				ValidationQuote::Simple(quote! {
					if let Err(e) = #custom_func_name(&self.#field_name, &context) {
					  errors.push(e);
					}
				}),
				true,
			)
		}
		m if m.path.is_ident("async_custom_with_context") => {
			let val: LitStr = m
				.value()
				.unwrap_or_else(|_| panic!("'async_custom_with_context' need a value"))
				.parse()
				.unwrap_or_else(|_| panic!("'async_custom_with_context' value should be a function name"));

			let custom_func_name: Path = val
				.parse()
				.unwrap_or_else(|_| panic!("'async_custom_with_context' value should be a function name"));

			(
				ValidationQuote::Simple(quote! {
					if let Err(e) = #custom_func_name(&self.#field_name, &context).await {
					  errors.push(e);
					}
				}),
				true,
			)
		}
		_ => (ValidationQuote::Simple(quote! {}), false),
	}
}
