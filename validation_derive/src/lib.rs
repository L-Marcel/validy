mod attributes;
mod core;
mod factories;
mod fields;
mod imports;
mod primitives;
mod types;

use std::cell::RefCell;

use crate::{
	attributes::get_attributes,
	core::{get_fields, get_fields_attributes},
	factories::core::get_factory,
	imports::ImportsSet,
	types::{Input, Output},
};

use proc_macro_error::{emit_error, proc_macro_error};
use syn::{DeriveInput, spanned::Spanned};

/// **Struct options:** `#[validate(context = <?type>, modify = <?bool>, payload = <?bool>, asyncronous = <?bool>)]`
/// * Configures global validation behavior, context injection, and serialization hooks.
///
/// **Special attributes:**: `#[special(<rule>, ...)]`
/// * `nested`: Validates the fields of a nested struct.
/// * `for_each(config?(from_item = <?type>, to_collection = <?type>, from_collection = <?type>), <rule>)`: Applies validation rules to every element in a collection.
/// * `from_type(value = <?type>)`: Defines a custom wrapper field type.
///
/// **Validation attributes:** `#[validate(<rule>, ...)]`
///
/// *Existence & Options:*
/// * `required(message = <?string>, code = <?string>)`: Changes the default message and code displayed when a field is missing.
/// * `is_some(message = <?string>, code = <?string>)`: Validates that an `Option` is `Some`.
/// * `is_none(message = <?string>, code = <?string>)`: Validates that an `Option` is `None`.
///
/// *String & Patterns:*
/// * `contains(slice = <string>, message = <?string>, code = <?string>)`: Validates that the string contains the specified substring.
/// * `email(message = <?string>, code = <?string>)`: Validates that the string follows a standard email format.
/// * `url(message = <?string>, code = <?string>)`: Validates that the string is a standard URL.
/// * `ip(message = <?string>, code = <?string>)`: Validates that the string is a valid IP address (v4 or v6).
/// * `ipv4(message = <?string>, code = <?string>)`: Validates that the string is a valid IPv4 address.
/// * `ipv6(message = <?string>, code = <?string>)`: Validates that the string is a valid IPv6 address.
/// * `pattern(pattern = <regex>, message = <?string>, code = <?string>)`: Validates that the string matches the provided Regex pattern.
/// * `suffix(suffix = <string>, message = <?string>, code = <?string>)`: Validates that the string ends with the specified suffix.
/// * `prefix(prefix = <string>, message = <?string>, code = <?string>)`: Validates that the string starts with the specified prefix.
///
/// *Numbers & Collections:*
/// * `range(range = <range>, message = <?string>, code = <?string>)`: Validates that the number falls within the specified numeric range.
/// * `length(range = <range>, message = <?string>, code = <?string>)`: Validates that the length (string or collection) is within limits.
/// * `any(items = <array>, message = <?string>, code = <?string>)`: Validates that the value is present in the allowed list (allowlist).
/// * `none(items = <array>, message = <?string>, code = <?string>)`: Validates that the value is NOT present in the forbidden list (blocklist).
///
/// *Date & Time:*
/// * `time(format = <string>, message = <?string>, code = <?string>)`: Validates that the string matches the specified time/date format.
/// * `naive_time(format = <string>, message = <?string>, code = <?string>)`: Validates that the string matches the specified naive time format.
/// * `after_now(accept_equals = <?bool>, message = <?string>, code = <?string>)`: Validates that the date/time is strictly after the current time.
/// * `before_now(accept_equals = <?bool>, message = <?string>, code = <?string>)`: Validates that the date/time is strictly before the current time.
/// * `now(ms_tolerance = <?int>, message = <?string>, code = <?string>)`: Validates that the date/time matches the current time within a tolerance (default: 500ms).
///
/// *Custom:*
/// * `inline(closure = <closure>, params = <?array>, message = <?string>, code = <?string>)`: Validates using a simple inline closure returning a boolean.
/// * `custom(function = <function>, params = <?array>)`: Validates using a custom function.
/// * `custom_with_context(function = <function>, params = <?array>)`: Validates using a custom function with access to the context.
/// * `async_custom(function = <function>, params = <?array>)`: Validates using a custom async function.
/// * `async_custom_with_context(function = <function>, params = <?array>)`: Validates using a custom async function with access to the context.
///
/// **Modification attributes:** `#[modify(<modifier>, ...)]`
///
/// *String & Cases:*
/// * `trim`: Removes whitespace from both ends of the string.
/// * `trim_start`: Removes whitespace from the start of the string.
/// * `trim_end`: Removes whitespace from the end of the string.
/// * `uppercase`: Converts all characters in the string to uppercase.
/// * `lowercase`: Converts all characters in the string to lowercase.
/// * `capitalize`: Capitalizes the first character of the string.
/// * `camel_case`: Converts the string to CamelCase (PascalCase).
/// * `lower_camel_case`: Converts the string to lowerCamelCase.
/// * `snake_case`: Converts the string to snake_case.
/// * `shouty_snake_case`: Converts the string to SHOUTY_SNAKE_CASE.
/// * `kebab_case`: Converts the string to kebab-case.
/// * `shouty_kebab_case`: Converts the string to SHOUTY-KEBAB-CASE.
/// * `train_case`: Converts the string to Train-Case.
///
/// *Custom:*
/// * `inline(closure = <closure>, params = <?array>)`: Modifies the value using an inline closure.
/// * `custom(function = <function>, params = <?array>)`: Modifies the value in-place using a custom function.
/// * `custom_with_context(function = <function>, params = <?array>)`: Modifies the value in-place using a custom function with context access.
/// * `async_custom(function = <function>, params = <?array>)`: Modifies the value in-place using a custom async function.
/// * `async_custom_with_context(function = <function>, params = <?array>)`: Modifies the value in-place using a custom async function with context access.
#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate, modify, special))]
pub fn validation_macro(input: Input) -> Output {
	let ast = syn::parse(input).unwrap();
	impl_validation_macro(&ast)
}

fn impl_validation_macro(ast: &DeriveInput) -> Output {
	let fields = get_fields(ast);
	let mut attributes = get_attributes(ast);
	let imports = RefCell::new(ImportsSet::new());

	if attributes.modify && attributes.payload {
		emit_error!(ast.span(), "payload implies modify");
	}

	attributes.modify = attributes.modify || attributes.payload;

	let factory = get_factory(&ast.ident, &attributes);
	let fields_attributes = get_fields_attributes(fields, factory.as_ref(), &attributes, &imports);
	factory.create(fields_attributes, &attributes, &imports)
}
