mod attributes;
mod core;
mod factories;
mod fields;
mod imports;
mod primitives;
mod types;

use crate::{
	attributes::get_attributes,
	core::{get_fields, get_operations},
	factories::core::get_factory,
	imports::{import_async_trait, import_validation},
	types::{Input, Output},
};

use proc_macro_error::{emit_error, proc_macro_error};
use syn::{DeriveInput, spanned::Spanned};

/// **Struct options:** `#[validate(asyncronous = <bool>, context = <type>, deserialize = <bool>, modify = <bool>)]`
/// * Configures global validation behavior, context injection, and serialization hooks.
///
/// **Validation attributes:** `#[validate(<rule>, ...)]`
/// * `nested`: Validates the fields of a nested struct.
/// * `for_each(<rule>, ...)`: Applies validation rules to every element in a collection.
/// * `required(<message>, <code>)`: Enforces that the value is present.
/// * `email(<message>, <code>)`: Validates that the string follows a standard email format.
/// * `range(<range>, <message>, <code>)`: Validates that the number falls within the specified numeric range.
/// * `length(<range>, <message>, <code>)`: Validates that the length (string or collection) is within limits.
/// * `suffix(<string>, <message>, <code>)`: Validates that the string ends with the specified suffix.
/// * `prefix(<string>, <message>, <code>)`: Validates that the string starts with the specified prefix.
/// * `pattern(<regex>, <message>, <code>)`: Validates that the string matches the provided Regex pattern.
/// * `url(<message>, <code>)`: Validates that the string is a standard URL.
/// * `ip(<message>, <code>)`: Validates that the string is a valid IP address (v4 or v6).
/// * `ipv4(<message>, <code>)`: Validates that the string is a valid IPv4 address.
/// * `ipv6(<message>, <code>)`: Validates that the string is a valid IPv6 address.
/// * `contains(<needle>, <message>, <code>)`: Validates that the collection or string contains the specified element/substring.
/// * `any(<list>, <message>, <code>)`: Validates that the value is present in the allowed list (allowlist).
/// * `none(<list>, <message>, <code>)`: Validates that the value is NOT present in the forbidden list (blocklist).
/// * `time(<format>, <message>, <code>)`: Validates that the string matches the specified time/date format.
/// * `after(<time>, <message>, <code>)`: Validates that the date/time is strictly after the specified value.
/// * `before(<time>, <message>, <code>)`: Validates that the date/time is strictly before the specified value.
/// * `inline(<function>, <message>, <code>)`: Validates using a simple inline closure returning a boolean.
/// * `custom(<function>)`: Validates using a custom function.
/// * `custom_with_context(<function>)`: Validates using a custom function with access to the context.
/// * `async_custom(<function>)`: Validates using a custom async function.
/// * `async_custom_with_context(<function>)`: Validates using a custom async function with access to the context.
///
/// **Modification attributes:** `#[modify(<modifier>, ...)]`
/// * `trim`: Removes whitespace from both ends of the string.
/// * `trim_start`: Removes whitespace from the start of the string.
/// * `trim_end`: Removes whitespace from the end of the string.
/// * `uppercase`: Converts all characters in the string to uppercase.
/// * `lowercase`: Converts all characters in the string to lowercase.
/// * `capitalize`: Capitalizes the first character of the string.
/// * `parse_from(<type>)`: Parses the value from a another type.
/// * `custom_parse(<function>)`: Parses the value using a custom transformation function.
/// * `custom_parse_with_context(<function>)`: Parses the value using a custom function with context access.
/// * `async_custom_parse(<function>)`: Parses the value using a custom async function.
/// * `async_custom_parse_with_context(<function>)`: Parses the value using a custom async function with context access.
/// * `custom(<function>)`: Modifies the value in-place using a custom function.
/// * `custom_with_context(<function>)`: Modifies the value in-place using a custom function with context access.
/// * `async_custom(<function>)`: Modifies the value in-place using a custom async function.
/// * `async_custom_with_context(<function>)`: Modifies the value in-place using a custom async function with context access.
#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate, modify, complex))]
pub fn validation_macro(input: Input) -> Output {
	let ast = syn::parse(input).unwrap();
	impl_validation_macro(&ast)
}

fn impl_validation_macro(ast: &DeriveInput) -> Output {
	let fields = get_fields(ast);
	let mut attributes = get_attributes(ast);

	if attributes.modify && attributes.payload {
		emit_error!(ast.span(), "payload implies modify");
	}

	attributes.modify = attributes.modify || attributes.payload;

	let factory = get_factory(&ast.ident, &attributes);
	let (operations, fields_attributes) = get_operations(fields, factory.as_ref(), &attributes);
	factory.create(operations, fields_attributes)
}
