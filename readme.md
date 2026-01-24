# Validy

*But, also modification.*

[![Status](https://github.com/L-Marcel/validy/actions/workflows/ci.yml/badge.svg)](https://github.com/L-Marcel/validy/actions/workflows/ci.yml)

A powerful and flexible Rust library based on procedural macros for `validation`, `modification`, and DTO (Data Transfer Object) handling. Designed to integrate seamlessly with `Axum`. Inspired by `Validator`, `Validify` and `Garde`.

- [📝 Installation](#-installation)
- [🚀 Quick Start](#-quick-start)
- [🔎 Validation Flow](#-validation-flow)
- [🎯 Work In Progress](#-work-in-progress)
- [🔌 Axum Integration](#-axum-integration)
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
  - [For `errors`](#for-errors)
  - [For `assertions`](#for-assertions)
- [📁 More Examples](#-more-examples)
- [🎁 For Developers](#-for-developers)

## 📝 Installation

Add with Cargo:

```
cargo add validy --features axum,email
```

Or add this to your Cargo.toml:

```toml
[dependencies]
validy = { version = "1.0.0", features = ["axum", "email"] }
```

## 🚀 Quick Start

The main entry point is the `#[derive(Validate)]` macro. It allows you to configure validations, modifications, and payload behaviors directly on your struct.

```rust
use crate::core::{errors::Error, services::user::UserService};
//-------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^ Well, it's my validation context.
// You will use your own when need pass a context.
use serde::Deserialize;
use std::sync::Arc;
use validy::core::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum)]
pub struct CreateUserExampleDTO {
	#[modify(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))] // Just change required message
	pub name: String,

	#[modify(trim)]
	#[validate(email("invalid email format", "bad_format"))]
	#[validate(async_custom_with_context(validate_unique_email))]
	// You can pass extra args.
	//#[validate(async_custom_with_context(validate_unique_email, [&wrapper.name]))]
	// If payload is false, you should replace 'wrapper' by 'self'.
	// Technically you can also access variables within the implementation, but I don't recommend it. 
	#[validate(inline(|_| true))] //Just an example.
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	pub email: String,
	
	// Rule's args order can be changed using the '=' operator.
	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	// However, args order is still the priority.
	//#[validate(length(3..=12, "size", message = "password must be between 3 and 12 characters"))]
	// Above, "size" is a message (which has been overridden, by the way).
	pub password: String,

	#[special(from_type(String))] // Id will be deserialized as Option<String>.
	#[modify(lowercase)] // You can modify or validade as String, if has some.
	#[modify(inline(|_| 3))] // You can parse to the final value type.
	#[validate(range(3..=12))] // And validade or modify again.
	pub dependent_id: u16,

	#[modify(trim)]
	#[validate(length(0..=254, "tag must not be more than 254 characters"))]
	#[modify(snake_case)]
	#[modify(custom(modify_tag))]
	pub tag: Option<String>, // Tag is really optional.
	
	#[special(from_type(RoleWrapper))] // Required to correctly define the wrapper field type.
	#[special(nested(Role, RoleWrapper))] // Required to correctly validate nested content.
	// The wrapper type and the rule `from_type` can be ignored when `payload` is disabled.
	//#[special(nested(Role))]
	pub role: Option<Role>, //Can be optional, or not.
	//pub role: Role,
}

// To pass a struct to nested validations, the struct needs `Default` derive.
#[derive(Debug, Deserialize, Default, Validate)]
#[validate(payload)]
pub struct Role {
	#[special(from_type(Vec<String>))]
	#[special(for_each( // You can validate or modify each item of collections.
 	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
    modify(inline(|x: &str| ::serde_json::from_str::<u32>(x).unwrap_or(0))), // Just another parse example.
    validate(inline(|x: &u32| *x > 1)), // Just a validation example.
 	  modify(inline(|x| x + 1))
	))]
	pub permissions: Vec<u32>,
}

// As a rule, the input is `(&field, &field_name)`.
// All custom rules also can be throw validation errors.
// Unfortunately, each modification has to return a new value, instead of changing the existing one. 
// This ensures that changes are only commited at the end of the validation process.
fn modify_tag(tag: &str, field_name: &str) -> (String, Option<ValidationError>) {
	("new_tag".to_string(), None)
}

// Custom functions can be async, instead sync.
// With context, or not. See `custom` and `custom_with_context`, `async_custom`,
// `async_custom_with_context` and `inline` rules.
async fn validate_unique_email(
	email: &str,
	field_name: &str,
	service: &Arc<dyn UserService>, // Only if has context.
	//name: &str                    // Example with extra args.
) -> Result<(), ValidationError> {
	let result = service.email_exists(email).await;

	match result {
		Ok(false) => Ok(()),
		Ok(true) => Err(ValidationError::builder()
			.with_field("email")
			.as_simple("unique")
			.with_message("e-mail must be unique")
			.build()
			.into()),
		Err(error) => {
			Err(ValidationError::builder()
				.with_field("email")
				.as_simple("internal")
				.with_message("internal error")
				.build()
				.into())
		}
	}
}
```

## 🔎 Validation Flow

You might not like it, but I took the liberty of naming things as I want. So, first, lets me show my glossary:

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

Almost all `rules` are executed in order from left to right and from top to bottom, according to their role group and definitions.

There is a cost to commit changes after all the `rules` have been met. When the `modify` or `payload` configuration attributes are enabled, a new copy of the changed value will be created after each modification.

In contrast, no primitive `rule` is asynchronous, therefore the `asynchronous` configuration attribute is only necessary to enable custom `rules`. The use of `context` is similar.

## 🎯 Work In Progress

- [] More test coverage.
- [] Failure mode.
- [] Typed multipart/form-data validation support.
  - [] file validation rules too (maybe).
- [] Validation rules for uuid.
- [] Validation rules for decimal (maybe).

## 🔌 Axum Integration

When enabling the `axum` feature the library automatically generates the `FromRequest` implementation for your `struct` with `axum` configuration attribute enabled. The automated flow:

- *Extract:* receives the JSON body.
- *Deserialize:* deserializes the body.
  - When the `payload` configuration attribute is enabled, the body will be deserialized as a `wrapper`.
  - The name of the `wrapper` struct is the name of the `payload` struct with the suffix `'Wrapper'`, for example: `CreateUserDTO` generates a public `wrapper` named `CreateUserDTOWrapper`.
  - The generated `wrapper` is left exposed for you to use.
- *Execute:* executes all the `rules`.
- *Convert:* if successful, passes the final struct to the `handler`.
- *Error Handling:* if any step fails, returns `Bad Request` with a structured list of errors.
  - When the `payload` configuration attribute is disabled, missing fields throws `Unprocessable Entity`.
  
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
	body: CreateUserDTO, // You can deconstruct too.
	// CreateUserDTO { name, email, password }: CreateUserDTO,
) -> Result<impl IntoResponse, Error> {
	let user = service.create(body.name, body.email, body.password).await?;
	Ok((StatusCode::CREATED, Json(UserDTO::from(user))))
}
```

Yes, it's beautiful.


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
| Validation | `Validate`, `AsyncValidate`, `ValidateWithContext<C>`, `SpecificValidateWithContext`, `AsyncValidateWithContext<C>` and  `SpecificAsyncValidateWithContext`. |
| Modification | `ValidateAndModificate`, `AsyncValidateAndModificate`, `ValidateAndModificateWithContext<C>`, `SpecificValidateAndModificateWithContext`, `AsyncValidateAndModificateWithContext<C>` and `SpecificAsyncValidateAndModificateWithContext`. |
| Parsing | `ValidateAndParse<W>`, `SpecificValidateAndParse`, `AsyncValidateAndParse<W>`, `SpecificAsyncValidateAndParse`, `ValidateAndParseWithContext<W, C>`, `SpecificValidateAndParseWithContext`, `AsyncValidateAndParseWithContext<W, C>` and  `SpecificAsyncValidateAndParseWithContext`. |
| Error | `IntoValidationError` |

## 🚩 Feature Flags

Crate behavior can be adjusted in Cargo.toml.

| **Feature** | **Description** | **Dependencies** |
| :-------- | :------- | :------- |
| `default` | `derive`, `validation`, `modification` | | 
| `all` | Enables all features. | |
| `derive` | Enables macro functionality. | `serde`, `validation_derive` |
| `validation` | Enables validation functions. Needed by almost all `derive` primitives validation rules. | |
| `modification` | Enables modification functions. Needed by almost all `derive` primitives modification rules. | `heck` |
| `email` | Enables `email` validation rule. | `email_address` |
| `pattern` | Enables `pattern` and `url` validation rules. Uses `moka` to cache `regex`. Cache can be configured calling `ValidationSettings::init(...)`. | `moka`, `regex` | 
| `ip` | Enables `ipI'm a busy and currently not very successful graduate student, so don't expect too much from me in terms of maintenance. But I did my best.` validation rule. | |
| `time` | Enables time validation rules. | `chrono` |
| `axum` | `derive` \| Enables axum integration. | `axum` |
| `macro_rules` | Enables macros for validation errors. | |
| `macro_rules_assertions` | Enables macros for assertions (tests). | `pretty_assertions` |

## 🚧 Validation Rules

Primitive rules of `#[validate(<rule>, ...)]` rule group.

> The '?' indicates that arg is optional.

### For `required` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `required`(message = <?string>, code = <?string>) | Changes the default message and code displayed when a field is missing. Requires that `payload` configuration attribute is enabled. |

### For `string` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `contains`(slice = \<string>, message = <?string>, code = <?string>) | Validates that the string contains the specified substring. |
| `email`(message = <?string>, code = <?string>) | Validates that the string follows a standard email format. |
| `url`(message = <?string>, code = <?string>) | Validates that the string is a standard URL. Finding goods regex patterns for URLs is so difficult and tedious. I decided to use the pattern `(http(s)?:\/\/.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)` related [here](https://stackoverflow.com/a/3809435). |
| `ip`(message = <?string>, code = <?string>) | Validates that the string is a valid IP address (v4 or v6). |
| `ipv4`(message = <?string>, code = <?string>) | Validates that the string is a valid IPv4 address. |
| `ipv6`(message = <?string>, code = <?string>) | Validates that the string is a valid IPv6 address. |
| `pattern`(pattern = \<regex>, message = <?string>, code = <?string>) | Validates that the string matches the provided Regex  pattern. |
| `suffix`(suffix = \<string>, message = <?string>, code = <?string>) | Validates that the string ends with the specified suffix. |
| `prefix`(prefix = \<string>, message = <?string>, code = <?string>) | Validates that the string starts with the specified prefix. |
| `length`(range = \<range>, message = <?string>, code = <?string>) | Validates that the length (string or collection) is within limits. |

### For `collection` or `single` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `length`(range = \<range>, message = <?string>, code = <?string>) | Validates that the length (string or collection) is within limits. |
| `allowlist`(mode = <"SINGLE" \| "COLLECTION">, items = \<array>, message = <?string>, code = <?string>) | Validates that the value or collection items is present in the allowed list (allowlist). |
| `blocklist`(mode = <"SINGLE" \| "COLLECTION">, items = \<array>, message = <?string>, code = <?string>) | Validates that the value or collection items is NOT present in the forbidden list (blocklist). |

### For `numbers` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `range`(range = \<range>, message = <?string>, code = <?string>) | Validates that the number falls within the specified numeric range. |

### For `date` or `time` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `time`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `DateTime<FixedOffset>` format. Not parse the string. |
| `naive_time`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `NaiveDateTime` format. Not parse the string. |
| `naive_date`(format = \<string>, message = <?string>, code = <?string>) | Validates that the string matches the specified `NaiveDate` format. Not parse the string. |
| `after_now`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` is strictly after the current time. |
| `before_now`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` is strictly before the current time. |
| `now`(ms_tolerance = <?int>, message = <?string>, code = <?string>) | Validates that the `DateTime<FixedOffset>` matches the current time within a tolerance (default: 500ms). |
| `after_today`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `NaiveDate` is strictly after the current day. |
| `before_today`(accept_equals = <?bool>, message = <?string>, code = <?string>) | Validates that the `NaiveDate` is strictly before the current day. |
| `today`(message = <?string>, code = <?string>) | Validates that the `NaiveDate matches the current day. |


### Custom rules

All with prefix `async_` requires that `asynchronous` configuration attribute is enabled. And all with suffix `_with_context` requires that `context` configuration attribute is defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>, message = <?string>, code = <?string>) | Validates using a simple inline  closure returning a boolean. |
| `custom`(function = \<function>, params = <?array>) | Validates using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Validates using a custom function with access to the context. |
| `async_custom`(function = \<function>, params = <?array>) | Validates using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Validates using a custom async function with access to the context. |

## 🔨 Modification Rules

Primitive rules of `#[modify(<rule>, ...)]` rule group. All requires that `payload` or `modify` configuration attributes are enabled.

> The '?' indicates that arg is optional.

### For `string` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `trim` | Removes whitespace from both ends of the string. |
| `trim_start` | Removes whitespace from the start of the string. |
| `trim_end` | Removes whitespace from the end of the string. |
| `uppercase` | Converts all characters in the string to uppercase. |
| `lowercase` | Converts all characters in the string to lowercase. |
| `capitalize` | Capitalizes the first character of the string. |
| `camel_case` | Converts the string to CamelCase (PascalCase). |
| `lower_camel_case` | Converts the string to lowerCamelCase. |
| `snake_case` | Converts the string to snake_case. |
| `shouty_snake_case` | Converts the string to SHOUTY_SNAKE_CASE. |
| `kebab_case` | Converts the string to kebab-case. |
| `shouty_kebab_case` | Converts the string to SHOUTY-KEBAB-CASE. |
| `train_case` | Converts the string to Train-Case. |

### For `date` or `time` fields

All these rules was created to be used with the special rule `#[special(from_type(String))]` before.

| **Rule** | **Description** |
| :-------- | :------- |
| `parse_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses that the string matches the specified time/date format. |
| `parse_naive_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses that the string matches the specified naive time format. |
| `parse_naive_date`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses that the string matches the specified naive date format. |

### Custom rules

All with prefix `async_` requires that `asynchronous` configuration attribute is enabled. And all with suffix `_with_context` requires that `context` configuration attribute is defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>) | Modifies the value using an inline closure. |
| `custom`(function = \<function>, params = <?array>) | Modifies the value in-place using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Modifies the value in-place using a custom function with context access. |
| `async_custom`(function = \<function>, params = <?array>) | Modifies the value in-place using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Modifies the value in-place using a custom async function with context access. |

## 🔧 Special Rules

Primitive rules of `#[special(<rule>, ...)]` rule group.

> The '?' indicates that arg is optional.

| **Rule** | **Description** |
| :-------- | :------- |
| `nested`(value = <type>, wrapper = <?type>) | Validates the fields of a nested struct. Warning: cyclical references can cause many problems. |
| `for_each`(config?(from_item = <?type>, to_collection = <?type>, from_collection = <?type>), \<rule>) | Applies validation rules  to every element in a collection. The arg `from_item` from optional `config` rule defines the type of each item of the collection. The arg `to_collection` defines the final type of the collection and the arg `from_collection` defines de initial type of the collection. Just `from_type` adapters to collections. |
| `from_type`(value = <?type>) | Need to be defined above and first all others rules. |

## 📐 Useful Macros

Sometimes, you might prefer to use macros to declare errors or assertions.

### For `errors`

All requires that `macro_rules` feature flag is enabled.

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

### For `assertions`
  
All requires that `macro_rules_assertions` feature flag is enabled.

```rust
let mut wrapper = TestWrapper::default();
let mut result = Test::validate_and_parse(&wrapper);
assert_errors!(result, wrapper, { // Wrapper is the input
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

If you need more references, you can use the [/tests](/tests) as an example.

## 🎁 For Developers

Well... You can run all tests with `cargo test-all` and see the `derive` macros's implementations running the script `expand.sh` (requires `cargo expand`). It will compile, generate and check all tests. I hope.

> I'm a busy and currently not very successful graduate student, so don't expect too much from me in terms of maintenance. But I did my best.
