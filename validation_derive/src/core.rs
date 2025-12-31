use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, meta::ParseNestedMeta, spanned::Spanned};

use crate::{
	attributes::ValidationAttributes,
	factories::core::AbstractValidationFactory,
	fields::FieldAttributes,
	primitives::{
		collections::{any::create_any, none::create_none},
		complexes::for_each::create_for_each,
		customs::{
			async_custom::create_async_custom, async_custom_with_context::create_async_custom_with_context,
			custom::create_custom, custom_with_context::create_custom_with_context,
		},
		format::{
			capitalize::create_capitalize, lowercase::create_lowercase, trim::create_trim, uppercase::create_uppercase,
		},
		ips::{ip::create_ip, ipv4::create_ipv4, ipv6::create_ipv6},
		patterns::{contains::create_contains, email::create_email, pattern::create_pattern, url::create_url},
		ranges::{length::create_length, range::create_range},
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
		.enumerate()
		.flat_map(|(index, field)| {
			let mut operations = Vec::<TokenStream>::new();

			let field_name = &field.ident;
			let field_type = &field.ty;

			let mut field_attributes = match field_name {
				Some(name) => FieldAttributes::from_named(field_type, name),
				None => {
					let index = Index {
						index: index as u32,
						span: field.span(),
					};

					FieldAttributes::from_unamed(field_type, &index)
				}
			};

			for attr in &field.attrs {
				if attr.path().is_ident("validate") {
					let _ = attr.parse_nested_meta(|meta| {
						let validation = get_validation_by_attr_macro(factory, meta, &mut field_attributes, attributes);
						operations.push(validation.clone());
						Ok(())
					});
				} else if attr.path().is_ident("modify") {
					let _ = attr.parse_nested_meta(|meta| {
						let operation = get_operation_by_attr_macro(factory, meta, &mut field_attributes, attributes);
						operations.push(operation.clone());
						Ok(())
					});
				} else if attr.path().is_ident("complex") {
					let _ = attr.parse_nested_meta(|meta| {
						let operation = get_complex_by_attr_macro(factory, meta, &mut field_attributes, attributes);
						operations.push(operation.clone());
						Ok(())
					});
				}
			}

			operations
		})
		.collect()
}

pub fn get_validation_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	match meta {
		//custom
		m if m.path.is_ident("custom") => create_custom(m.input, field),
		m if m.path.is_ident("custom_with_context") => create_custom_with_context(m.input, field, attributes),
		m if m.path.is_ident("async_custom") => create_async_custom(m.input, field, attributes),
		m if m.path.is_ident("async_custom_with_context") => {
			create_async_custom_with_context(m.input, field, attributes)
		}

		//ip
		m if m.path.is_ident("ip") => create_ip(m.input, field),
		m if m.path.is_ident("ipv4") => create_ipv4(m.input, field),
		m if m.path.is_ident("ipv6") => create_ipv6(m.input, field),

		//pattern
		m if m.path.is_ident("pattern") => create_pattern(m.input, field),
		m if m.path.is_ident("url") => create_url(m.input, field),
		m if m.path.is_ident("email") => create_email(m.input, field),

		//range
		m if m.path.is_ident("range") => create_range(m.input, field),
		m if m.path.is_ident("length") => create_length(m.input, field),
		m if m.path.is_ident("contains") => create_contains(m.input, field),

		//todo
		m if m.path.is_ident("any") => create_any(m.input, field),
		m if m.path.is_ident("none") => create_none(m.input, field),

		m if m.path.is_ident("inline") => create_contains(m.input, field),
		m if m.path.is_ident("prefix") => create_contains(m.input, field),
		m if m.path.is_ident("suffix") => create_contains(m.input, field),
		m if m.path.is_ident("before") => create_contains(m.input, field),
		m if m.path.is_ident("after") => create_contains(m.input, field),
		m if m.path.is_ident("time") => create_contains(m.input, field),
		m if m.path.is_ident("required") => create_contains(m.input, field),

		_ => quote! {},
	}
}

pub fn get_operation_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	if !attributes.modify {
		emit_error!(meta.input.span(), "requires modify attribute");
		return quote! {};
	}

	match meta {
		m if m.path.is_ident("trim") => create_trim(field),
		m if m.path.is_ident("uppercase") => create_uppercase(field),
		m if m.path.is_ident("lowercase") => create_lowercase(field),
		m if m.path.is_ident("capitalize") => create_capitalize(field),
		_ => quote! {},
	}

	// modify(camel_case)
	// modify(lower_camel_case)
	// modify(snake_case)
	// modify(shouty_snake_case)
	// modify(kebab_case)
	// modify(shouty_kebab_case)
	// modify(train_case)

	// modify(custom)
	// modify(async_custom)
	// modify(custom_with_context)
	// modify(async_custom_with_context)
}

pub fn get_complex_by_attr_macro(
	factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	// complex(parse)

	match meta {
		m if m.path.is_ident("nested") => factory.create_nested(field),
		m if m.path.is_ident("for_each") => create_for_each(factory, m, field, attributes),
		_ => quote! {},
	}
}
