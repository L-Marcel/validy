# Validy

*More than just validation.*

[![Status](https://github.com/L-Marcel/validy/actions/workflows/ci.yml/badge.svg)](https://github.com/L-Marcel/validy/actions/workflows/ci.yml)

A powerful and flexible Rust library based on procedural macros for `validation`, `modification`, and DTO (Data Transfer Object) handling. Designed to integrate seamlessly with `Axum`. Inspired by `Validator`, `Validify`, and `Garde`.

- [📝 Installation](#-installation)
- [🚀 Quick Start](#-quick-start)
- [🔎 Validation Flow](#-validation-flow)
  - [Implementations](#implementations)
- [🎯 Work In Progress](#-work-in-progress)
- [🔌 Axum Integration](#-axum-integration)
  - [Customizing the failure `status code`](#customizing-the-failure-status-code)
- [🧩 Manual Usage](#-manual-usage)
  - [Available traits](#available-traits)
- [🚩 Feature Flags](#-feature-flags)
- [🚧 Validation Rules](#-validation-rules)
  - [For `required` fields](#for-required-fields)
  - [For `string` fields](#for-string-fields)
  - [For `collection` or `single` fields](#for-collection-or-single-fields)
  - [For `numbers` fields](#for-numbers-fields)
  - [For `date` or `time` fields](#for-date-or-time-fields)
  - [Custom rules](#custom-rules)
- [🔨 Modification Rules](#-modification-rules)
  - [For `string` fields](#for-string-fields-1)
  - [For `date` or `time` fields](#for-date-or-time-fields-1)
  - [Custom rules](#custom-rules-1)
- [🔧 Special Rules](#-special-rules)
- [📐 Useful Macros](#-useful-macros)
  - [For `error` handling](#for-error-handling)
  - [For `test` assertions](#for-test-assertions)
- [📁 More Examples](#-more-examples)
- [🎁 For Developers](#-for-developers)

## 📝 Installation

Add with Cargo:

```
cargo add validy --features axum,email
```

## 🚀 Quick Start

The main entry point is the `#[derive(Validate)]` macro. It allows you to configure validations, modifications, and payload behaviors directly on your struct.

```rust
use crate::core::{errors::Error, services::user::UserService};
//-------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^ This is my validation context.
// You can use your own type when you need to pass a context.
use serde::Deserialize;
use std::sync::Arc;
use validy::core::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum)]
pub struct CreateUserExampleDTO {
	#[modify(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))] // Just changes the 'required' message.
	pub name: String,

	#[modify(trim)]
	#[validate(email("invalid email format", "bad_format"))]
	#[validate(async_custom_with_context(validate_unique_email))]
	// You can pass extra arguments.
	//#[validate(async_custom_with_context(validate_unique_email, [&wrapper.name]))]
	// If 'payload' is false, you should replace 'wrapper' with 'self'.
	// Technically you can also access variables within the implementation, but I don't recommend it. 
	#[validate(inline(|_| true))] // Just an example.
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	pub email: String,
	
	// The order of a rule's arguments can be changed using the '=' operator.
	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	// However, positional argument order is still the priority.
	//#[validate(length(3..=12, "size", message = "password must be between 3 and 12 characters"))]
	// In the line above, "size" is treated as the message argument (which is then immediately overridden).
	pub password: String,

	#[special(from_type(String))] // 'dependent_id' will be deserialized as Option<String>.
	#[modify(lowercase)] // You can modify or validate it as a String, if it has a value.
	#[modify(inline(|_| 3))] // You can then parse it to the final value type.
	#[validate(range(3..=12))] // And validate or modify it again.
	pub dependent_id: u16,

	#[modify(trim)]
	#[validate(length(0..=254, "tag must not be more than 254 characters"))]
	#[modify(snake_case)]
	#[modify(custom(modify_tag))]
	pub tag: Option<String>, // 'tag' is truly optional.
	
	#[special(from_type(RoleWrapper))] // Required to correctly define the wrapper field type.
	#[special(nested(Role, RoleWrapper))] // Required to correctly validate nested content.
	// The wrapper type and the 'from_type' rule can be ignored when 'payload' is disabled.
	//#[special(nested(Role))]
	pub role: Option<Role>, // Can be optional or required.
	//pub role: Role,
}

// To use a struct in nested validations, it needs to derive 'Default'.
#[derive(Debug, Deserialize, Default, Validate)]
#[validate(payload, axum)]
pub struct Role {
	#[special(from_type(Vec<String>))]
	#[validate(length(1..=2))]
	#[special(for_each( // You can validate or modify each item in a collection.
 	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
    modify(inline(|x: &str| ::serde_json::from_str::<u32>(x).unwrap_or(0))), // Just another parse example.
    validate(inline(|x: &u32| *x > 1)), // Just a validation example.
 	  modify(inline(|x| x + 1))
	))]
	pub permissions: Vec<u32>,
	
	#[special(from_type(Vec<String>))]
	#[special(for_each(
	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
		modify(inline(|x: &str| ::serde_json::from_str::<u32>(x).unwrap_or(0))),
	  validate(inline(|x: &u32| *x > 1)),
		modify(inline(|x| x + 1))
	))]
	pub alt_permissions: Vec<u32>,
}

// As a rule, the input for custom functions is '(&field, &field_name)'.
// All custom modification rules can also throw validation errors.
// Unfortunately, each modification has to return a new value instead of changing the existing one in-place.
// This ensures that changes are only committed at the end of the validation process.
fn modify_tag(tag: &str, _field_name: &str) -> (String, Option<ValidationError>) {
	("new_tag".to_string(), None)
}

// Custom functions can be async instead of sync.
// With context, or not. See 'custom', 'custom_with_context', 'async_custom',
// 'async_custom_with_context', and 'inline' rules.
async fn validate_unique_email(
	email: &str,
	field_name: &str,
	service: &Arc<dyn UserService>, // Only if context is provided.
	//name: &str                    // Example with extra arguments.
) -> Result<(), ValidationError> {
	let result = service.email_exists(email).await;

	match result {
		Ok(false) => Ok(()),
		Ok(true) => Err(ValidationError::builder()
			.with_field(field_name.to_string())
			.as_simple("unique")
			.with_message("e-mail must be unique")
			.build()
			.into()),
		Err(_) => { // Simplified error handling
			Err(ValidationError::builder()
				.with_field(field_name.to_string())
				.as_simple("internal")
				.with_message("internal error")
				.build()
				.into())
		}
	}
}
```

## 🔎 Validation Flow

You might not like it, but I took the liberty of naming things as I see fit. So, first, let me show you my glossary:

```rust
#[derive(Debug, Deserialize, Validate)]
//vvvvvvvv Configuration
#[validate(asynchronous, context = Arc<dyn UserService>, payload)]
//---------^^^^^^^^^^^^ Configuration attribute
pub struct CreateUserExampleDTO {
    //vvvvvv Rule group
    #[modify(trim, lowercase)]
    //-------^^^^ Rule
    #[validate(length(3..=120, "name must be between 3 and 120 characters"))]
    //----------------^^^^^^^ Rule arg 'range' value
    pub name: String,
    //-------------------------------vvvvvv Rule arg 'code' value
    #[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
    //------------------------^^^^ Rule arg 'code' declaration
    pub password: String,
}
```

Almost all `rules` are executed from left to right and top to bottom, according to their rule group and definition order.

### Implementations

There is a cost to committing changes after all `rules` have been met. When the `modify` or `payload` configuration attributes are enabled, a clone of the value is created after each modification. Some validation rules also need to clone values.

In contrast, no primitive `rule` is asynchronous. Therefore, the `asynchronous` configuration attribute is only necessary to enable custom async `rules`. The use of `context` is similar.

## 🎯 Work In Progress

Some of these features are available now, but are only partially finished. I will document them fully once they are complete.

- [ ] More test coverage.
- [x] Custom validation status code.
- [ ] Failure mode.
  - The current default is `FailOncePerField` (covered by the tests).
- [ ] Typed multipart/form-data validation support.
  - [ ] File validation rules (maybe).
- [x] Validation rules for uuid.
- [ ] Validation rules for decimal (maybe).
- [ ] Better macro documentation.

## 🔌 Axum Integration

When you enable the `axum` feature, the library automatically generates the `FromRequest` implementation for your `struct` if it has the `axum` configuration attribute enabled. The automated flow is as follows:

- *Extract:* receives the JSON body.
- *Deserialize:* deserializes the body.
  - When the `payload` configuration attribute is enabled, the body is deserialized into a `wrapper`.
  - The name of the `wrapper` struct is the name of the `payload` struct with the suffix `'Wrapper'`. For example, `CreateUserDTO` generates a public `wrapper` named `CreateUserDTOWrapper`.
  - The generated `wrapper` is left exposed for you to use.
- *Execute:* executes all the `rules`.
- *Convert:* if successful, passes the final struct to the `handler`.
- *Error Handling:* if any step fails, returns `Bad Request` with a structured list of errors.
  - When the `payload` configuration attribute is disabled, missing fields throw an `Unprocessable Entity` error.
  
See an example:

```rust
#[derive(Debug, Deserialize, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum)]
pub struct CreateUserDTO {
	#[modify(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	pub name: String,

	#[modify(trim)]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	#[validate(email("invalid email format"))]
	#[validate(async_custom_with_context(validate_unique_email))]
	pub email: String,

	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,
}

#[debug_handler]
pub async fn create_user(
	State(service): State<Arc<dyn UserService>>,
	body: CreateUserDTO, // You can also deconstruct it.
	// CreateUserDTO { name, email, password }: CreateUserDTO,
) -> Result<impl IntoResponse, Error> {
	let user = service.create(body.name, body.email, body.password).await?;
	Ok((StatusCode::CREATED, Json(UserDTO::from(user))))
}
```

Yes, it's beautiful.

### Customizing the failure `status code`

You can change the HTTP status code returned on validation failure:

```rust
ValidationSettings::set_failure_status_code(StatusCode::BAD_REQUEST);
```

This method is `thread-safe`. The default status code is `BAD_REQUEST`.

## 🧩 Manual Usage

The derive macros implement specific traits for your structs. To call methods like `.validate()`, `.async_validate()`, or `::validate_and_parse(...)`, you must import the corresponding traits into your scope.

```rust
use validy::core::{Validate, AsyncValidate, ValidateAndParse};

// Or just import the prelude
use validy::core::*;
```

### Available traits

| **Category** | **Traits** |
| :-------- | :------- |
| Validation | `Validate`, `AsyncValidate`, `ValidateWithContext<C>`, `SpecificValidateWithContext`, `AsyncValidateWithContext<C>`, and `SpecificAsyncValidateWithContext`. |
| Modification | `ValidateAndModificate`, `AsyncValidateAndModificate`, `ValidateAndModificateWithContext<C>`, `SpecificValidateAndModificateWithContext`, `AsyncValidateAndModificateWithContext<C>`, and `SpecificAsyncValidateAndModificateWithContext`. |
| Parsing | `ValidateAndParse<W>`, `SpecificValidateAndParse`, `AsyncValidateAndParse<W>`, `SpecificAsyncValidateAndParse`, `ValidateAndParseWithContext<W, C>`, `SpecificValidateAndParseWithContext`, `AsyncValidateAndParseWithContext<W, C>`, and `SpecificAsyncValidateAndParseWithContext`. |
| Error | `IntoValidationError` |

## 🚩 Feature Flags

The crate's behavior can be adjusted in your `Cargo.toml`.

| **Feature** | **Description** | **Dependencies** |
| :-------- | :------- | :------- |
| `default` | `derive`, `validation`, `modification` | | 
| `all` | Enables all features. | |
| `derive` | Enables macro functionality. | `serde`, `validation_derive` |
| `validation` | Enables validation functions. Needed by almost all primitive `derive` validation rules. | |
| `modification` | Enables modification functions. Needed by almost all primitive `derive` modification rules. | `heck` |
| `uuid` | Enables `uuid` rules. | `uuid` |
| `email` | Enables email rule. | `email_address` |
| `pattern` | Enables `pattern` and `url` rules. Uses `moka` to cache compiled `regex` patterns. The cache can be configured by calling `ValidationSettings::set_regex_cache(...)`. | `moka`, `regex` | 
| `ip` | Enables ip rules. | |
| `time` | Enables time rules. | `chrono` |
| `axum` | Enables Axum integration. | `axum`, `derive` |
| `axum_multipart` | Enables multipart support. | `axum_typed_multipart`, `axum` |
| `macro_rules` | Enables macros for validation errors. | |
| `macro_rules_assertions` | Enables macros for assertions (tests). | `pretty_assertions` |

## 🚧 Validation Rules

Primitive rules for the `#[validate(...)]` attribute.

> The '?' indicates that the argument is optional.

### For `required` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `required`(message = <?string>, code = <?string>) | Overrides the default message and code for a missing field. This rule requires the `payload` attribute to be enabled on the struct. |

### For `string` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `contains`(slice = \<string>, message = <?string>, code = <?string>) | Validates that the string contains the specified substring. |
| `uuid`(message = <?string>, code = <?string>) | Validates that the string is a valid UUID. This does not parse the string. |
| `email`(message = <?string>, code = <?string>) | Validates that the string follows a standard email format. |
| `url`(message = <?string>, code = <?string>) | Validates that the string is a standard URL. Finding good regex patterns for URLs is difficult and tedious, so I used the pattern `(http(s)?:\/\/.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)` found [here](https://stackoverflow.com/a/3809435). |
| `ip`(message = <?string>, code = <?string>) | Validates that the string is a valid IP address (v4 or v6). |
| `ipv4`(message = <?string>, code = <?string>) | Validates that the string is a valid IPv4 address. |
| `ipv6`(message = <?string>, code = <?string>) | Validates that the string is a valid IPv6 address. |
| `pattern`(pattern = \<regex>, message = <?string>, code = <?string>) | Validates that the string matches the provided Regex pattern. |
| `suffix`(suffix = \<string>, message = <?string>, code = <?string>) | Validates that the string ends with the specified suffix. |
| `prefix`(prefix = \<string>, message = <?string>, code = <?string>) | Validates that the string starts with the specified prefix. |
| `length`(range = \<range>, message = <?string>, code = <?string>) | Validates that the length of a string or collection is within the specified range. |

### For `collection` or `single` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `length`(range = \<range>, message = <?string>, code = <?string>) | Validates that the length of a string or collection is within the specified range. |
| `allowlist`(mode = <"SINGLE" \| "COLLECTION">, items = \<array>, message = <?string>, code = <?string>) | Validates that the value or collection items are present in the allowlist. |
| `blocklist`(mode = <"SINGLE" \| "COLLECTION">, items = \<array>, message = <?string>, code = <?string>) | Validates that the value or collection items are NOT present in the blocklist. |

### For `numbers` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `range`(range = \<range>, message = <?string>, code = <?string>) | Validates that the number falls within the specified numeric range. |

### For `date` or `time` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `time`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `DateTime<FixedOffset>` format. This does not parse the string. |
| `naive_time`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `NaiveDateTime` format. This does not parse the string. |
| `naive_date`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `NaiveDate` format. This does not parse the string. |
| `after_now`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` is strictly after the current time. |
| `before_now`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` is strictly before the current time. |
| `now`(ms_tolerance = <?int>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` matches the current time within a tolerance (default: 500ms). |
| `after_today`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `NaiveDate` is strictly after the current day. |
| `before_today`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `NaiveDate` is strictly before the current day. |
| `today`(message = <?string>, code = <?string>) | Validates that the `NaiveDate` matches the current day. |


### Custom rules

All rules prefixed with `async_` require the `asynchronous` configuration attribute to be enabled. All rules suffixed with `_with_context` require the `context` configuration attribute to be defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>, message = <?string>, code = <?string>) | Validates using a simple inline closure returning a boolean. |
| `custom`(function = \<function>, params = <?array>) | Validates using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Validates using a custom function with access to the context. |
| `async_custom`(function = \<function>, params = <?array>) | Validates using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Validates using a custom async function with access to the context. |

## 🔨 Modification Rules

Primitive rules for the `#[modify(...)]` attribute. These all require either the `payload` or `modify` attribute to be enabled on the struct.

> The '?' indicates that the argument is optional.

### For `string` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `parse_uuid` | Validates that a string is a valid UUID and parses it. |
| `trim` | Removes whitespace from both ends of the string. |
| `trim_start` | Removes whitespace from the start of the string. |
| `trim_end` | Removes whitespace from the end of the string. |
| `uppercase` | Converts all characters in the string to uppercase. |
| `lowercase` | Converts all characters in the string to lowercase. |
| `capitalize` | Capitalizes the first character of each word in the string. |
| `camel_case` | Converts the string to CamelCase (PascalCase). |
| `lower_camel_case` | Converts the string to lowerCamelCase. |
| `snake_case` | Converts the string to snake_case. |
| `shouty_snake_case` | Converts the string to SHOUTY_SNAKE_CASE. |
| `kebab_case` | Converts the string to kebab-case. |
| `shouty_kebab_case` | Converts the string to SHOUTY-KEBAB-CASE. |
| `train_case` | Converts the string to Train-Case. |

### For `date` or `time` fields

All of these rules were created to be used with `#[special(from_type(String))]` declared before them.

| **Rule** | **Description** |
| :-------- | :------- |
| `parse_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `DateTime<FixedOffset>` matching the specified format. |
| `parse_naive_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `NaiveDateTime` matching the specified format. |
| `parse_naive_date`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `NaiveDate` matching the specified format. |

### Custom rules

All rules prefixed with `async_` require the `asynchronous` configuration attribute to be enabled. All rules suffixed with `_with_context` require the `context` configuration attribute to be defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>) | Modifies the value using an inline closure. |
| `custom`(function = \<function>, params = <?array>) | Modifies the value using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Modifies the value using a custom function with context access. |
| `async_custom`(function = \<function>, params = <?array>) | Modifies the value using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Modifies the value using a custom async function with context access. |

## 🔧 Special Rules

Primitive rules for the `#[special(...)]` attribute.

> The '?' indicates that the argument is optional.

| **Rule** | **Description** |
| :-------- | :------- |
| `nested`(value = <type>, wrapper = <?type>) | Validates the fields of a nested struct. Warning: cyclical references can cause compilation issues. |
| `ignore` | Ignores any validation or modification rule. |
| `for_each`(config?(from_item = <?type>, to_collection = <?type>, from_collection = <?type>), \<rule>) | Applies validation rules to every element in a collection. The `from_item` arg from the optional `config` rule defines the type of each collection item. The `to_collection` arg defines the final type of the collection, and the `from_collection` arg defines the initial type. It's like a `from_type` adapter for collections. |
| `from_type`(value = <?type>) | Defines the type of the field in the wrapper. Must be defined before all other rules on a field. |

## 📐 Useful Macros

Sometimes, you might prefer to use macros to declare errors or assertions.

### For `error` handling
  
All require the `macro_rules` feature flag to be enabled.

```rust
// SimpleValidationError
let error = validation_error!(field.to_string(), "custom_code", "custom message");
```

```rust
// SimpleValidationError
let error = validation_error!(field.to_string(), "custom_code");
```

```rust
// ValidationErrors
let errors = validation_errors! {
  "a" => ("custom_code", "custom message"),
	"b" => ("nested", validation_errors! {
	  "c" => ("custom_code", "custom message")
	})
};
```

```rust
// NestedValidationError
let error = nested_validation_error!(
	field.to_string(),
	"custom_code",
	validation_errors! {
    "a" => ("custom_code", "custom message"),
	}
);
```

### For `test` assertions
  
All require the `macro_rules_assertions` feature flag to be enabled.

```rust
let mut wrapper = TestWrapper::default();
let mut result = Test::validate_and_parse(&wrapper);
assert_errors!(result, wrapper, { // 'wrapper' is the input
	"a" => ("required", "is required"),
});
```

```rust
let result = test.validate_and_modificate();
assert_validation!(result, test);
assert_modification!(test.b, Some(expected.to_string()), test);
```

```rust
result = Test::validate_and_parse(&wrapper);
assert_parsed!(result, wrapper, Test { a: *expected, b: None });
```

## 📁 More Examples

If you need more references, you can use the `tests` directory as a reference.

## 🎁 For Developers

You can run all tests with `cargo test --all --all-features`. To see the generated code from the `derive` macros, you can run the `expand.sh` script (this requires `cargo expand`). It will compile, generate, and check all tests.

> This is a personal project maintained by a graduate student. Maintenance may be limited, but I do my best to keep it in good shape.
