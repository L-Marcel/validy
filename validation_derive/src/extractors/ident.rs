use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, meta::ParseNestedMeta};

use crate::{
	ValidationAttributes,
	factories::core::AbstractValidationFactory,
	primitives::{
		async_custom::create_async_custom, contains::create_contains, custom::create_custom, email::create_email,
		ip::create_ip, ipv4::create_ipv4, ipv6::create_ipv6, length::create_length, pattern::create_pattern,
		range::create_range, url::create_url,
	},
};

pub fn get_fields(input: &DeriveInput) -> &Fields {
	if let Data::Struct(data) = &input.data {
		&data.fields
	} else {
		panic!("validation only supports structs!");
	}
}

pub fn get_operations(
	fields: &Fields,
	factory: &dyn AbstractValidationFactory,
	attributes: &ValidationAttributes,
) -> Vec<TokenStream> {
	fields
		.iter()
		.flat_map(|field| {
			let field_name = &field.ident;
			let field_type = &field.ty;

			let mut operations = Vec::<TokenStream>::new();

			for attr in &field.attrs {
				if attr.path().is_ident("validate") {
					let _ = attr.parse_nested_meta(|meta| {
						let validation =
							get_validation_by_attr_macro(factory, field_name, field_type, meta, attributes);
						operations.push(validation.clone());
						Ok(())
					});
				} else if attr.path().is_ident("modify") {
					let _ = attr.parse_nested_meta(|meta| {
						let operation = get_operation_by_attr_macro(factory, field_name, field_type, meta);
						operations.push(operation.clone());
						Ok(())
					});
				}
			}

			operations
		})
		.collect()
}

fn get_validation_by_attr_macro(
	factory: &dyn AbstractValidationFactory,
	field_name: &Option<Ident>,
	field_type: &Type,
	meta: ParseNestedMeta<'_>,
	attributes: &ValidationAttributes,
) -> TokenStream {
	match meta {
		//m if factory.meta_is_custom(&m) => factory.create_custom(field_name, m),
		m if m.path.is_ident("custom") => create_custom(field_name, m.input),
		m if m.path.is_ident("async_custom") => create_async_custom(field_name, m.input, attributes),
		//m if factory.meta_is_invalid_custom(&m) => factory.create_invalid_custom_error(m),
		m if m.path.is_ident("nested") => factory.create_nested(field_name, field_type),

		m if m.path.is_ident("range") => create_range(field_name, m.input),
		m if m.path.is_ident("length") => create_length(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("email") => create_email(field_name, m.input).unwrap_or_else(|| quote! {}),

		m if m.path.is_ident("pattern") => create_pattern(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("url") => create_url(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("ip") => create_ip(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("ipv4") => create_ipv4(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("ipv6") => create_ipv6(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("contains") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),

		//todo
		m if m.path.is_ident("any") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("none") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),

		m if m.path.is_ident("inline") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("prefix") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("suffix") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("before") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("after") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("time") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("required") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		m if m.path.is_ident("for_each") => create_contains(field_name, m.input).unwrap_or_else(|| quote! {}),
		_ => quote! {},
	}
}

fn get_operation_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	_field_name: &Option<Ident>,
	_field_type: &Type,
	_meta: ParseNestedMeta<'_>,
) -> TokenStream {
	// modify(trim)
	// modify(parse)
	// modify(uppercase)
	// modify(lowercase)
	// modify(captalize)
	// modify(custom)
	// modify(async_custom)
	// modify(custom_with_context)
	// modify(async_custom_with_context)
	// modify(nested)

	// payload
	quote! {}
}

pub fn get_ident_from_nested_meta(meta: &ParseNestedMeta<'_>) -> Option<Ident> {
	let mut ident: Option<Ident> = None;

	let _ = meta.parse_nested_meta(|meta| {
		let path_ident = meta.path.get_ident();

		if let Some(value) = path_ident {
			ident = Some(value.clone())
		}

		Ok(())
	});

	ident
}
