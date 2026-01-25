mod attributes;
mod core;
mod factories;
mod fields;
mod imports;
mod primitives;
mod types;

use std::cell::RefCell;

use crate::{
	attributes::{get_attributes, get_others_attributes, get_others_attributes_by_fields},
	core::{get_fields, get_fields_attributes},
	factories::core::get_factory,
	imports::ImportsSet,
	types::{Input, Output},
};

use proc_macro_error::{emit_error, proc_macro_error};
use syn::{DeriveInput, spanned::Spanned};

/// **Struct options:** `#[validate(context = <?type>, modify = <?bool>, payload = <?bool>, asynchronous = <?bool>)]`
/// * Configures global validation behavior, context injection, and serialization hooks.
///
/// **Repository:** `https://github.com/L-Marcel/validy`
///
/// **Special attributes:**: `#[special(<rule>, ...)]`
/// * `nested`(value = <type>, wrapper = <?type>): Validates the fields of a nested struct. Warning: cyclical references can cause compilation issues.
/// * `for_each`(config?(from_item = <?type>, to_collection = <?type>, from_collection = <?type>), <rule>): Applies validation rules to every element in a collection. The `from_item` arg from the optional `config` rule defines the type of each collection item. The `to_collection` arg defines the final type of the collection, and the `from_collection` arg defines the initial type. It's like a `from_type` adapter for collections.
/// * `from_type`(value = <?type>): Defines the type of the field in the wrapper. Must be defined before all other rules on a field.
///
/// **Validation attributes:** `#[validate(<rule>, ...)]`
///
/// *For `required` fields:*
/// * `required`(message = <?string>, code = <?string>): Overrides the default message and code for a missing field. This rule requires the `payload` attribute to be enabled on the struct.
///
/// *For `string` fields:*
/// * `contains`(slice = <string>, message = <?string>, code = <?string>): Validates that the string contains the specified substring.
/// * `uuid`(message = <?string>, code = <?string>): Validates that the string is a valid UUID. This does not parse the string.
/// * `email`(message = <?string>, code = <?string>): Validates that the string follows a standard email format.
/// * `url`(message = <?string>, code = <?string>): Validates that the string is a standard URL. Finding good regex patterns for URLs is difficult and tedious, so I used the pattern `(http(s)?:\/\/.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)` found [here](https://stackoverflow.com/a/3809435).
/// * `ip`(message = <?string>, code = <?string>): Validates that the string is a valid IP address (v4 or v6).
/// * `ipv4`(message = <?string>, code = <?string>): Validates that the string is a valid IPv4 address.
/// * `ipv6`(message = <?string>, code = <?string>): Validates that the string is a valid IPv6 address.
/// * `pattern`(pattern = <regex>, message = <?string>, code = <?string>): Validates that the string matches the provided Regex pattern.
/// * `suffix`(suffix = <string>, message = <?string>, code = <?string>): Validates that the string ends with the specified suffix.
/// * `prefix`(prefix = <string>, message = <?string>, code = <?string>): Validates that the string starts with the specified prefix.
/// * `length`(range = <range>, message = <?string>, code = <?string>): Validates that the length of a string or collection is within the specified range.
///
/// *For `collection` or `single` fields:*
/// * `length`(range = <range>, message = <?string>, code = <?string>): Validates that the length of a string or collection is within the specified range.
/// * `allowlist`(mode = <"SINGLE" \| "COLLECTION">, items = <array>, message = <?string>, code = <?string>): Validates that the value or collection items are present in the allowlist.
/// * `blocklist`(mode = <"SINGLE" \| "COLLECTION">, items = <array>, message = <?string>, code = <?string>): Validates that the value or collection items are NOT present in the blocklist.
///
/// *For `numbers` fields:*
/// * `range`(range = <range>, message = <?string>, code = <?string>): Validates that the number falls within the specified numeric range.
///
/// *For `date` or `time` fields:*
/// * `time`(format = <string>, message = <?string>, code = <?string>): Validates that the string matches the specified `DateTime<FixedOffset>` format. This does not parse the string.
/// * `naive_time`(format = <string>, message = <?string>, code = <?string>): Validates that the string matches the specified `NaiveDateTime` format. This does not parse the string.
/// * `naive_date`(format = <string>, message = <?string>, code = <?string>): Validates that the string matches the specified `NaiveDate` format. This does not parse the string.
/// * `after_now`(accept_equals = <?bool>, message = <?string>, code = <?string>): Validates that the `DateTime<FixedOffset>` is strictly after the current time.
/// * `before_now`(accept_equals = <?bool>, message = <?string>, code = <?string>): Validates that the `DateTime<FixedOffset>` is strictly before the current time.
/// * `now`(ms_tolerance = <?int>, message = <?string>, code = <?string>): Validates that the `DateTime<FixedOffset>` matches the current time within a tolerance (default: 500ms).
/// * `after_today`(accept_equals = <?bool>, message = <?string>, code = <?string>): Validates that the `NaiveDate` is strictly after the current day.
/// * `before_today`(accept_equals = <?bool>, message = <?string>, code = <?string>): Validates that the `NaiveDate` is strictly before the current day.
/// * `today`(message = <?string>, code = <?string>): Validates that the `NaiveDate` matches the current day.
///
/// *Custom rules:*
/// * `inline`(closure = <closure>, params = <?array>, message = <?string>, code = <?string>): Validates using a simple inline closure returning a boolean.
/// * `custom`(function = <function>, params = <?array>): Validates using a custom function.
/// * `custom_with_context`(function = <function>, params = <?array>): Validates using a custom function with access to the context.
/// * `async_custom`(function = <function>, params = <?array>): Validates using a custom async function.
/// * `async_custom_with_context`(function = <function>, params = <?array>): Validates using a custom async function with access to the context.
///
/// **Modification attributes:** `#[modify(<modifier>, ...)]`
///
/// *For `string` fields:*
/// * `parse_uuid`: Validates that a string is a valid UUID and parses it.
/// * `trim`: Removes whitespace from both ends of the string.
/// * `trim_start`: Removes whitespace from the start of the string.
/// * `trim_end`: Removes whitespace from the end of the string.
/// * `uppercase`: Converts all characters in the string to uppercase.
/// * `lowercase`: Converts all characters in the string to lowercase.
/// * `capitalize`: Capitalizes the first character of each word in the string.
/// * `camel_case`: Converts the string to CamelCase (PascalCase).
/// * `lower_camel_case`: Converts the string to lowerCamelCase.
/// * `snake_case`: Converts the string to snake_case.
/// * `shouty_snake_case`: Converts the string to SHOUTY_SNAKE_CASE.
/// * `kebab_case`: Converts the string to kebab-case.
/// * `shouty_kebab_case`: Converts the string to SHOUTY-KEBAB-CASE.
/// * `train_case`: Converts the string to Train-Case.
///
/// *For `date` or `time` fields:*
/// * `parse_time`(format = <string>, message = <?string>, code = <?string>): Validates and parses a string into a `DateTime<FixedOffset>` matching the specified format.
/// * `parse_naive_time`(format = <string>, message = <?string>, code = <?string>): Validates and parses a string into a `NaiveDateTime` matching the specified format.
/// * `parse_naive_date`(format = <string>, message = <?string>, code = <?string>): Validates and parses a string into a `NaiveDate` matching the specified format.
///
/// *Custom rules:*
/// * `inline`(closure = <closure>, params = <?array>): Modifies the value using an inline closure.
/// * `custom`(function = <function>, params = <?array>): Modifies the value using a custom function.
/// * `custom_with_context`(function = <function>, params = <?array>): Modifies the value using a custom function with context access.
/// * `async_custom`(function = <function>, params = <?array>): Modifies the value using a custom async function.
/// * `async_custom_with_context`(function = <function>, params = <?array>): Modifies the value using a custom async function with context access.
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

	let struct_attrs = get_others_attributes(&ast.attrs);
	let factory = get_factory(&ast.ident, &attributes);
	let fields_attributes = get_fields_attributes(fields, factory.as_ref(), &attributes, &imports);
	let fields_attrs = get_others_attributes_by_fields(fields);
	factory.create(fields_attributes, &attributes, &imports, struct_attrs, fields_attrs)
}
