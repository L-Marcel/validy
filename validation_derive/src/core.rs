use std::cell::RefCell;

use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, meta::ParseNestedMeta, spanned::Spanned};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	factories::core::AbstractValidationFactory,
	fields::FieldAttributes,
	primitives::{
		collections::{allowlist::create_allowlist, blocklist::create_blocklist},
		customs::{
			modification::{
				async_custom::create_async_custom_modification,
				async_custom_with_context::create_async_custom_with_context_modification,
				custom::create_custom_modification, custom_with_context::create_custom_with_context_modification,
			},
			parsing::{
				async_custom::create_async_custom_parse,
				async_custom_with_context::create_async_custom_with_context_parse, custom::create_custom_parse,
				custom_with_context::create_custom_with_context_parse,
			},
			validation::{
				async_custom::create_async_custom, async_custom_with_context::create_async_custom_with_context,
				custom::create_custom, custom_with_context::create_custom_with_context,
			},
		},
		field_datas::{
			field_content_type::create_field_content_type, field_file_name::create_field_file_name,
			field_name::create_field_name,
		},
		format::{
			camel_case::create_camel_case, capitalize::create_capitalize, kebab_case::create_kebab_case,
			lower_camel_case::create_lower_camel_case, lowercase::create_lowercase,
			shouty_kebab_case::create_shouty_kebab_case, shouty_snake_case::create_shouty_snake_case,
			snake_case::create_snake_case, train_case::create_train_case, trim::create_trim, trim_end::create_trim_end,
			trim_start::create_trim_start, uppercase::create_uppercase,
		},
		inlines::{
			inline_modification::create_inline_modification, inline_parse::create_inline_parse,
			inline_validation::create_inline_validation,
		},
		ips::{
			ip::create_ip, ipv4::create_ipv4, ipv6::create_ipv6, parse_ip::create_ip_parse,
			parse_ipv4::create_ipv4_parse, parse_ipv6::create_ipv6_parse,
		},
		option::required::create_required,
		patterns::{
			contains::create_contains, email::create_email, pattern::create_pattern, prefix::create_prefix,
			suffix::create_suffix, url::create_url,
		},
		ranges::{length::create_length, range::create_range},
		specials::{for_each::create_for_each, from_type::create_from_type, ignore::create_ignore},
		time::{
			after_now::create_after_now, after_today::create_after_today, before_now::create_before_now,
			before_today::create_before_today, default_time::create_time, naive_date::create_naive_date,
			naive_time::create_naive_time, now::create_now, parse_naive_date::create_naive_date_parse,
			parse_naive_time::create_naive_time_parse, parse_time::create_time_parse, today::create_today,
		},
		uuids::{parse_uuid::create_uuid_parse, uuid::create_uuid},
	},
};

pub fn get_fields(input: &DeriveInput) -> &Fields {
	if let Data::Struct(data) = &input.data {
		&data.fields
	} else {
		panic!("validation only supports structs!");
	}
}

pub fn get_fields_attributes(
	fields: &Fields,
	factory: &dyn AbstractValidationFactory,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> Vec<FieldAttributes> {
	let mut fields_attributes = Vec::<FieldAttributes>::new();

	fields.iter().enumerate().for_each(|(index, field): (usize, &Field)| {
		let field_name = &field.ident;
		let field_type = &field.ty;

		let mut field_attributes = match field_name {
			Some(name) => FieldAttributes::from_named(field_type, name, attributes),
			None => {
				let index = Index {
					index: index as u32,
					span: field.span(),
				};

				FieldAttributes::from_unamed(field_type, &index, attributes)
			}
		};

		for attr in &field.attrs {
			if attr.path().is_ident("validate")
				&& let Err(error) = attr.parse_nested_meta(|meta| {
					let validation =
						get_validate_by_attr_macro(factory, meta, &mut field_attributes, attributes, imports);
					field_attributes.add_operation(validation.clone());
					Ok(())
				}) {
				emit_error!(error.span(), error.to_string());
			} else if attr.path().is_ident("modificate")
				&& let Err(error) = attr.parse_nested_meta(|meta| {
					let operation =
						get_modificate_by_attr_macro(factory, meta, &mut field_attributes, attributes, imports);
					field_attributes.add_operation(operation.clone());
					Ok(())
				}) {
				emit_error!(error.span(), error.to_string());
			} else if attr.path().is_ident("parse")
				&& let Err(error) = attr.parse_nested_meta(|meta| {
					let operation = get_parse_by_attr_macro(factory, meta, &mut field_attributes, attributes, imports);
					field_attributes.add_operation(operation.clone());
					Ok(())
				}) {
				emit_error!(error.span(), error.to_string());
			} else if attr.path().is_ident("special")
				&& let Err(error) = attr.parse_nested_meta(|meta| {
					let operation =
						get_special_by_attr_macro(factory, meta, &mut field_attributes, attributes, imports);
					field_attributes.add_operation(operation.clone());
					Ok(())
				}) {
				emit_error!(error.span(), error.to_string());
			};
		}

		fields_attributes.push(field_attributes);
	});

	fields_attributes
}

pub fn get_validate_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	match meta {
		m if m.path.is_ident("required") => create_required(m.input, field, attributes),
		m if m.path.is_ident("inline") => create_inline_validation(m.input, field),
		m if m.path.is_ident("custom") => create_custom(m.input, field),
		m if m.path.is_ident("custom_with_context") => create_custom_with_context(m.input, field, attributes),
		m if m.path.is_ident("async_custom") => create_async_custom(m.input, field, attributes),
		m if m.path.is_ident("async_custom_with_context") => {
			create_async_custom_with_context(m.input, field, attributes)
		}
		m if m.path.is_ident("ip") => create_ip(m.input, field, imports),
		m if m.path.is_ident("ipv4") => create_ipv4(m.input, field, imports),
		m if m.path.is_ident("ipv6") => create_ipv6(m.input, field, imports),
		m if m.path.is_ident("pattern") => create_pattern(m.input, field, imports),
		m if m.path.is_ident("uuid") => create_uuid(m.input, field, imports),
		m if m.path.is_ident("url") => create_url(m.input, field, imports),
		m if m.path.is_ident("email") => create_email(m.input, field, imports),
		m if m.path.is_ident("prefix") => create_prefix(m.input, field, imports),
		m if m.path.is_ident("suffix") => create_suffix(m.input, field, imports),
		m if m.path.is_ident("range") => create_range(m.input, field, imports),
		m if m.path.is_ident("length") => create_length(m.input, field, imports),
		m if m.path.is_ident("contains") => create_contains(m.input, field, imports),
		m if m.path.is_ident("allowlist") => create_allowlist(m.input, field, imports),
		m if m.path.is_ident("blocklist") => create_blocklist(m.input, field, imports),
		m if m.path.is_ident("time") => create_time(m.input, field, imports),
		m if m.path.is_ident("before_now") => create_before_now(m.input, field, imports),
		m if m.path.is_ident("after_now") => create_after_now(m.input, field, imports),
		m if m.path.is_ident("naive_time") => create_naive_time(m.input, field, imports),
		m if m.path.is_ident("now") => create_now(m.input, field, imports),
		m if m.path.is_ident("before_today") => create_before_today(m.input, field, imports),
		m if m.path.is_ident("after_today") => create_after_today(m.input, field, imports),
		m if m.path.is_ident("today") => create_today(m.input, field, imports),
		m if m.path.is_ident("naive_date") => create_naive_date(m.input, field, imports),
		m if m.path.is_ident("field_content_type") => create_field_content_type(m.input, field, imports),
		m if m.path.is_ident("field_file_name") => create_field_file_name(m.input, field, imports),
		m if m.path.is_ident("field_name") => create_field_name(m.input, field, imports),
		_ => {
			emit_error!(meta.input.span(), "unknown value");
			quote! {}
		}
	}
}

pub fn get_modificate_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	if !attributes.modificate {
		emit_error!(meta.input.span(), "requires modificate attribute");
		return quote! {};
	}

	match meta {
		m if m.path.is_ident("custom") => create_custom_modification(m.input, field),
		m if m.path.is_ident("custom_with_context") => {
			create_custom_with_context_modification(m.input, field, attributes)
		}
		m if m.path.is_ident("async_custom") => create_async_custom_modification(m.input, field, attributes),
		m if m.path.is_ident("async_custom_with_context") => {
			create_async_custom_with_context_modification(m.input, field, attributes)
		}
		m if m.path.is_ident("trim") => create_trim(field),
		m if m.path.is_ident("trim_end") => create_trim_end(field),
		m if m.path.is_ident("trim_start") => create_trim_start(field),
		m if m.path.is_ident("uppercase") => create_uppercase(field),
		m if m.path.is_ident("lowercase") => create_lowercase(field),
		m if m.path.is_ident("capitalize") => create_capitalize(field, imports),
		m if m.path.is_ident("camel_case") => create_camel_case(field, imports),
		m if m.path.is_ident("lower_camel_case") => create_lower_camel_case(field, imports),
		m if m.path.is_ident("snake_case") => create_snake_case(field, imports),
		m if m.path.is_ident("shouty_snake_case") => create_shouty_snake_case(field, imports),
		m if m.path.is_ident("kebab_case") => create_kebab_case(field, imports),
		m if m.path.is_ident("shouty_kebab_case") => create_shouty_kebab_case(field, imports),
		m if m.path.is_ident("train_case") => create_train_case(field, imports),
		m if m.path.is_ident("inline") => create_inline_modification(m.input, field),
		_ => {
			emit_error!(meta.input.span(), "unknown value");
			quote! {}
		}
	}
}

pub fn get_parse_by_attr_macro(
	_factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	if !attributes.payload {
		emit_error!(meta.input.span(), "requires payload attribute");
		return quote! {};
	}

	match meta {
		m if m.path.is_ident("custom") => create_custom_parse(m.input, field),
		m if m.path.is_ident("custom_with_context") => create_custom_with_context_parse(m.input, field, attributes),
		m if m.path.is_ident("async_custom") => create_async_custom_parse(m.input, field, attributes),
		m if m.path.is_ident("async_custom_with_context") => {
			create_async_custom_with_context_parse(m.input, field, attributes)
		}
		m if m.path.is_ident("ip") => create_ip_parse(m.input, field, imports),
		m if m.path.is_ident("ipv4") => create_ipv4_parse(m.input, field, imports),
		m if m.path.is_ident("ipv6") => create_ipv6_parse(m.input, field, imports),
		m if m.path.is_ident("uuid") => create_uuid_parse(m.input, field, imports),
		m if m.path.is_ident("time") => create_time_parse(m.input, field, imports),
		m if m.path.is_ident("naive_time") => create_naive_time_parse(m.input, field, imports),
		m if m.path.is_ident("naive_date") => create_naive_date_parse(m.input, field, imports),
		m if m.path.is_ident("inline") => create_inline_parse(m.input, field),
		_ => {
			emit_error!(meta.input.span(), "unknown value");
			quote! {}
		}
	}
}

pub fn get_special_by_attr_macro(
	factory: &dyn AbstractValidationFactory,
	meta: ParseNestedMeta<'_>,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
	imports: &RefCell<ImportsSet>,
) -> TokenStream {
	match meta {
		m if m.path.is_ident("nested") => factory.create_nested(m.input, field),
		m if m.path.is_ident("from_type") => create_from_type(m.input, field, attributes),
		m if m.path.is_ident("for_each") => create_for_each(factory, m, field, attributes, imports),
		m if m.path.is_ident("ignore") => create_ignore(m.input, field, attributes),
		_ => {
			emit_error!(meta.input.span(), "unknown value");
			quote! {}
		}
	}
}
