# Validy

*More than just validation.*

[![Crates.io](https://img.shields.io/crates/v/validy.svg)](https://crates.io/crates/validy)
[![Status](https://github.com/L-Marcel/validy/actions/workflows/ci.yml/badge.svg)](https://github.com/L-Marcel/validy/actions/workflows/ci.yml)

A powerful and flexible Rust library based on procedural macros for `validation`, `modification`, and DTO (Data Transfer Object) handling. Designed to integrate seamlessly with `Axum`. Inspired by `Validator`, `Validify`, and `Garde`.

- [📝 Installation](#-installation)
- [🚀 Quick Start](#-quick-start)
- [📓 Glossary](#-glossary)
- [🔎 About Implementations](#-about-implementations)
  - [Failure modes](#failure-modes)
  - [Caching regex](#caching-regex)
- [🔌 Axum Integration](#-axum-integration)
  - [Customizing the failure `status code`](#customizing-the-failure-status-code)
  - [Multipart support](#multipart-support)
- [🧩 Manual Usage](#-manual-usage)
  - [Available traits](#available-traits)
- [🚩 Feature Flags](#-feature-flags)
- [🚧 Validation Rules](#-validation-rules)
  - [For `required` fields](#for-required-fields)
  - [For `string` fields](#for-string-fields)
  - [For `collection` or `single` fields](#for-collection-or-single-fields)
  - [For `numbers` fields](#for-numbers-fields)
  - [For `date` or `time` fields](#for-date-or-time-fields)
  - [For multipart `field data` fields](#for-multipart-field-data-fields)
  - [Custom rules](#custom-rules)
- [🔨 Modification Rules](#-modification-rules)
  - [For `string` fields](#for-string-fields-1)
  - [Custom rules](#custom-rules-1)
- [🔧 Parsing Rules](#-parsing-rules)
  - [For `uuid` fields](#for-uuid-fields)
  - [For `date` or `time` fields](#for-date-or-time-fields-1)
  - [Custom rules](#custom-rules-2)
- [🔮 Special Rules](#-special-rules)
- [📨 Wrappers](#-wrappers)
- [📐 Useful Macros](#-useful-macros)
  - [For `error` handling](#for-error-handling)
  - [For `test` assertions](#for-test-assertions)
- [💝 Complete Example](#-complete-example)
- [📁 More Examples](#-more-examples)
- [🎯 Work In Progress](#-work-in-progress)
- [🎁 For Developers](#-for-developers)

## 📝 Installation

Add with Cargo:

```bash
cargo add validy --features axum,email
```

## 🚀 Quick Start

The main entry point is the `#[derive(Validate)]` macro. It allows you to configure validations, modifications and parses behaviors directly on your struct. You also can use `#[wrapper_derive(...)]` to apply derive macros on [📨 Wrappers](#-wrappers).

```rust
use validy::core::{Validate, ValidateAndParse, ValidationError};
use validy::{assert_errors, assert_parsed};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, axum)]
#[wrapper_derive(Clone)]
pub struct CreateUserDTO {
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))]
	pub name: String,

	#[modificate(trim)]
	#[validate(email("invalid email format", "bad_format"))]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	pub email: String,

	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,

	#[special(from_type(String))]
	#[modificate(lowercase)]
	#[parse(inline(|x: String| x.parse::<u16>().unwrap_or(0)))]
	#[validate(range(3..=12))]
	pub dependent_id: u16,

	#[modificate(trim)]
	#[validate(length(0..=254, "code must not be more than 254 characters"))]
	#[modificate(snake_case)]
	#[modificate(custom(modificate_code))]
	pub code: Option<String>,
}

fn modificate_code(code: &mut String, _field_name: &str) -> Result<(), ValidationError> {
	*code = code.clone() + "_code";
	Ok(())
}

#[test]
fn should_validate() {
	let mut wrapper = CreateUserDTOWrapper {
		name: None,
		email: Some("test@gmail.com".to_string()),
		password: None,
		dependent_id: Some("3".to_string()),
		code: None,
	};

	let result = CreateUserDTO::validate_and_parse(wrapper.clone());

	assert_errors!(result, wrapper.clone(), {
	  "name" => ("required", "name is required"),
	  "password" => ("required", "is required")
	});

	wrapper.name = Some("test".to_string());
	wrapper.password = Some("test".to_string());
	wrapper.code = Some("code test".to_string());

	let result = CreateUserDTO::validate_and_parse(wrapper.clone());

	assert_parsed!(
		result,
		wrapper,
		CreateUserDTO {
			name: "test".to_string(),
			email: "test@gmail.com".to_string(),
			password: "test".to_string(),
			dependent_id: 3,
			code: Some("code_test_code".to_string()),
		}
	);
}
```

If that example isn't enough, check [📁 More Examples](#-more-examples).

## 📓 Glossary

I've made some naming that might escape the standard, so I think this might be helpful:

```rust
use validy::core::Validate;

#[derive(Validate)]
//vvvvvvvv Struct attribute
#[validate(asynchronous, context = bool, payload)]
//---------^^^^^^^^^^^^ Configuration attribute
pub struct CreateUserExampleDTO {
  //vvvvvvvvvv Field attribute
  #[modificate(trim, lowercase)]
  //-----------^^^^ Rule
  #[validate(length(3..=120, "name must be between 3 and 120 characters"))]
  //----------------^^^^^^^ Rule arg 'range' value
  pub name: String,
  //-------------------------------vvvvvv Rule arg 'code' value
  #[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
  //------------------------^^^^ Rule arg 'code' declaration
  pub password: String,
}
```

## 🔎 About Implementations

Almost all `rules` are executed from left to right and top to bottom, according to their field attribute and definition order.

I decided to avoid unnecessary `.clone()` calls for performance. Practically all rules only use references, the exceptions are the `allowlist` and `blocklist` rules, which need to clone the items (the field to be validated does not need to be cloned). Additionally, the `regex` rule and some rules with patterns needs to clone the `Arc` pointer from the cache.

It was also inevitable that the `parse` field attribute returns new values.

### Failure modes

Currently, there are four failure modes available. However, only one is covered by the tests (I will expand the coverage later). The options are:

- `FailureMode::FailOncePerField` 
  - Currently the standard value, and also covered by the tests.
  - As soon as an error is caught in a field, it moves on to another field.
- `FailureMode::FailFast` 
  - As soon as an error is caught, it stops validation and throws it. 
  - It is expected to be the most performant mode for this reason.
- `FailureMode::LastFailPerField` 
  - Keeps only the last error caught for each field.
- `FailureMode::FullFail` 
  - Captures all errors.

You can change the default failure mode calling:

```rust
use validy::settings::{FailureMode, ValidationSettings};

ValidationSettings::set_failure_mode(FailureMode::FailFast);
assert_eq!(ValidationSettings::get_failure_mode(), FailureMode::FailFast);
```

This method is `thread-safe`. Alternatively:

```rust
use validy::core::Validate;
use std::fmt::Debug;

#[derive(Debug, Validate)]
#[validate(payload, axum, failure_mode = FailFast)]
//------------------------^^^^^^^^^^^^^^^^^^^^^^^ Overrides the settings
pub struct CreateUserDTO {
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	pub name: String,
	
	//...
}
```

### Caching regex

Build regex for each request is slow. When `pattern` feature is enabled, not only are rules that use regex available, but also their cache settings. You can change calling:

```rust
use validy::settings::ValidationSettings;
use moka::sync::Cache;
use regex::Regex;
use std::{borrow::Cow, sync::Arc};

let cache = Cache::<Cow<'static, str>, Arc<Regex>>::builder()
	.max_capacity(100)
	.initial_capacity(10)
	.build();

ValidationSettings::set_regex_cache(cache);
let _ = ValidationSettings::get_regex_cache();
```

This method is `thread-safe`. The default value is what is shown in this example.

## 🔌 Axum Integration

When you enable the `axum` feature, the library automatically generates the `FromRequest` implementation for your `struct` if it has the `axum` configuration attribute enabled. The automated flow is as follows:

- *Extract:* receives the body.
- *Deserialize:* deserializes the body.
  - When the `payload` configuration attribute is enabled, the body is deserialized into [📨 Wrapper](#-wrappers).
- *Execute:* executes all `rules`.
- *Convert:* if successful, passes the final struct to the `handler`.
- *Error Handling:* if any step fails, returns `Bad Request` by default with the errors.

See an example:

```rust
use axum::{Json, extract::State, http::StatusCode, response::{Response, IntoResponse}};
use validy::core::{Validate, ValidateAndParse, ValidationError};
use std::{sync::Arc, fmt::Debug};

#[derive(Debug, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum)]
pub struct CreateUserDTO {
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	pub name: String,

	#[modificate(trim)]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	#[validate(email("invalid email format"))]
	#[validate(async_custom_with_context(validate_unique_email))]
	pub email: String,

	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,
}

pub async fn create_user(
	State(service): State<Arc<dyn UserService>>,
	CreateUserDTO { name, email, password }: CreateUserDTO,
) -> Result<Response, Response> {
	service.create(name, email, password).await?;
	Ok(StatusCode::CREATED.into_response())
}

async fn validate_unique_email(
	email: &str,
	_field_name: &str,
	service: &Arc<dyn UserService>,
) -> Result<(), ValidationError> {
	let result = service.email_exists(email).await;

	match result {
		Ok(false) => Ok(()),
		Ok(true) => Err(ValidationError::builder()
			.with_field("email")
			.as_simple("unique")
			.with_message("email already in use")
			.build()
			.into()),
		Err(_) => {
			Err(ValidationError::builder()
				.with_field("email")
				.as_simple("internal error")
				.with_message("It wasn't possible to verify if the email is unique")
				.build()
				.into())
		}
	}
}

#[async_trait::async_trait]
pub trait UserService: Send + Sync + Debug {
	async fn create(&self, name: String, email: String, password: String) -> Result<(), Response>;
	async fn email_exists(&self, email: &str) -> Result<bool, Response>;
	//...
}
```

Yes, it's beautiful.

### Customizing the failure `status code`

You can change the HTTP status code returned on validation failure:

```rust
use validy::settings::ValidationSettings;
use axum::http::StatusCode;

ValidationSettings::set_failure_status_code(StatusCode::UNPROCESSABLE_ENTITY);
assert_eq!(ValidationSettings::get_failure_status_code(), StatusCode::UNPROCESSABLE_ENTITY);

// For multipart
ValidationSettings::set_failure_multipart_status_code(StatusCode::UNPROCESSABLE_ENTITY);
assert_eq!(ValidationSettings::get_failure_multipart_status_code(), StatusCode::UNPROCESSABLE_ENTITY);
```

This method is `thread-safe`. The default status code is `BAD_REQUEST`.

### Multipart support

When you enable the `axum_multipart` feature, the library automatically generates the `FromRequest` implementation for your `struct` with `axum_typed_multipart` if it has the `multipart` configuration attribute enabled. But you still need to add `TryFromMultipart` macro derive if `payload` is disabled.

```rust
use axum::{Json, extract::State, http::StatusCode, response::{Response, IntoResponse}};
use validy::core::{Validate, ValidateAndParse, ValidationError};
use std::{sync::Arc, fmt::Debug};
use tempfile::NamedTempFile;
use axum_typed_multipart::{FieldData};

#[derive(Debug, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum, multipart)]
pub struct CreateUserDTO {
  #[wrapper_attribute(form_data(limit = "10MB"))]
  #[validate(field_content_type(r"^(image/.*)$"))] //requires `axum_multipart_field_data` feature yet
  pub avatar: FieldData<NamedTempFile>,
  
  #[wrapper_attribute(form_data(field_name = "user_name"))]
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	pub name: String,

	#[modificate(trim)]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	#[validate(email("invalid email format"))]
	#[validate(async_custom_with_context(validate_unique_email))]
	pub email: String,

	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,
}

pub async fn create_user(
	State(service): State<Arc<dyn UserService>>,
	CreateUserDTO { avatar, name, email, password }: CreateUserDTO,
) -> Result<Response, Response> {
	service.create(avatar, name, email, password).await?;
	Ok(StatusCode::CREATED.into_response())
}

async fn validate_unique_email(
	email: &str,
	_field_name: &str,
	service: &Arc<dyn UserService>,
) -> Result<(), ValidationError> {
	let result = service.email_exists(email).await;

	match result {
		Ok(false) => Ok(()),
		Ok(true) => Err(ValidationError::builder()
			.with_field("email")
			.as_simple("unique")
			.with_message("email already in use")
			.build()
			.into()),
		Err(_) => {
			Err(ValidationError::builder()
				.with_field("email")
				.as_simple("internal error")
				.with_message("It wasn't possible to verify if the email is unique")
				.build()
				.into())
		}
	}
}

#[async_trait::async_trait]
pub trait UserService: Send + Sync + Debug {
	async fn create(&self, avatar: FieldData<NamedTempFile>, name: String, email: String, password: String) -> Result<(), Response>;
	async fn email_exists(&self, email: &str) -> Result<bool, Response>;
	//...
}
```

Yes, it's beautiful too.

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
| `default` | `derive`, `validation`, `modification`, `parsing` | | 
| `all` | Enables all features. | |
| `derive` | Enables macro functionality. | `dep:serde`, `dep:validation_derive` |
| `validation` | Enables validation functions. Needed by almost all not custom or inline validation rules. | |
| `modification` | Enables modification functions. Needed by almost all not custom or inline modification rules. | `dep:heck` |
| `parsing` | Enables parsing functions. Needed by all not custom or inline parsing rules. | |
| `uuid` | Enables `uuid` rules. | `dep:uuid` |
| `email` | Enables email rule. | `dep:email_address` |
| `pattern` | Enables `pattern` and `url` rules. Uses `moka` to cache compiled `regex` patterns. The cache can be configured by calling `ValidationSettings::set_regex_cache(...)`. | `dep:moka`, `dep:regex` | 
| `ip` | Enables ip rules. | |
| `time` | Enables time rules. | `dep:chrono` |
| `axum` | Enables Axum integration. | `dep:axum`, `derive` |
| `axum_multipart` | Enables multipart support. | `axum_typed_multipart`, `axum` |
| `axum_multipart_field_data` | Enables multipart field data rules. | `axum_multipart`, `pattern` |
| `macro_rules` | Enables macros for validation errors. | |
| `macro_rules_assertions` | Enables macros for assertions (tests). | `dep:pretty_assertions` |

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

### For multipart `field data` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `field_name`(pattern = \<regex>, message = <?string>, code = <?string>) | Validates that the field name matches the provided Regex pattern. |
| `field_file_name`(pattern = \<regex>, message = <?string>, code = <?string>) | Validates that the field file name matches the provided Regex pattern. |
| `field_content_type`(pattern = \<regex>, message = <?string>, code = <?string>) | Validates that the field content type matches the provided Regex pattern. |

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

Primitive rules for the `#[modificate(...)]` attribute. These all require either the `payload` or `modificate` attribute to be enabled on the struct.

> The '?' indicates that the argument is optional.

### For `string` fields

| **Rule** | **Description** |
| :-------- | :------- |
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

### Custom rules

All rules prefixed with `async_` require the `asynchronous` configuration attribute to be enabled. All rules suffixed with `_with_context` require the `context` configuration attribute to be defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>) | Modifies the value using an inline closure. |
| `custom`(function = \<function>, params = <?array>) | Modifies the value using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Modifies the value using a custom function with context access. |
| `async_custom`(function = \<function>, params = <?array>) | Modifies the value using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Modifies the value using a custom async function with context access. |

## 🔧 Parsing Rules

Primitive rules for the `#[parse(...)]` attribute. These all require either the `payload` attribute to be enabled on the struct.

> The '?' indicates that the argument is optional.

### For `date` or `time` fields

All of these rules were created to be used with `#[special(from_type(String))]` declared before them.

| **Rule** | **Description** |
| :-------- | :------- |
| `parse_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `DateTime<FixedOffset>` matching the specified format. |
| `parse_naive_time`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `NaiveDateTime` matching the specified format. |
| `parse_naive_date`(format = \<string>, message = <?string>, code = <?string>) | Validates and parses a string into a `NaiveDate` matching the specified format. |

### For `uuid` fields

| **Rule** | **Description** |
| :-------- | :------- |
| `parse_uuid` | Validates and parses a string into a UUID. |

### Custom rules

All rules prefixed with `async_` require the `asynchronous` configuration attribute to be enabled. All rules suffixed with `_with_context` require the `context` configuration attribute to be defined.

| **Rule** | **Description** |
| :-------- | :------- |
| `inline`(closure = \<closure>, params = <?array>) | Parses the value using an inline closure. |
| `custom`(function = \<function>, params = <?array>) | Parses the value using a custom function. |
| `custom_with_context`(function = \<function>, params = <?array>) | Parses the value using a custom function with context access. |
| `async_custom`(function = \<function>, params = <?array>) | Parses the value using a custom async function. |
| `async_custom_with_context`(function = \<function>, params = <?array>) | Parses the value using a custom async function with context access. |

## 🔮 Special Rules

Primitive rules for the `#[special(...)]` attribute.

> The '?' indicates that the argument is optional.

| **Rule** | **Description** |
| :-------- | :------- |
| `nested`(value = <type>, wrapper = <?type>, code = <?string>) | Validates the fields of a nested struct. Warning: cyclical references can cause compilation issues. |
| `ignore` | Ignores any validation or modification rule. |
| `for_each`(config?(from_item = <?type>, to_collection = <?type>, from_collection = <?type>), \<rule>) | Applies validation rules to every element in a collection. The `from_item` arg from the optional `config` rule defines the type of each collection item. The `to_collection` arg defines the final type of the collection, and the `from_collection` arg defines the initial type. It's like a `from_type` adapter for collections. |
| `from_type`(value = <?type>) | Defines the type of the field in the wrapper. Must be defined before all other rules on a field. |

## 📨 Wrappers

Wrappers are generated structs similar to the original struct where all fields are covered with `Option`. They all have the `Default` derive macros by default. When the `multipart` configuration attribute is enabled, they also has `TryFromMultipart` derive macro, otherwise, they has `Deserialize` derive macro.

The name of the wrapper struct is the name of the origional struct with the suffix 'Wrapper'. For example, `CreateUserDTO` generates a public wrapper named `CreateUserDTOWrapper`. The generated wrapper is left exposed for you to use. You also can use `#[wrapper_derive(...)]` struct attribute in the origional struct to apply derive macros on the wrapper and `#[wrapper_attribute(...)]` attribute in the origional struct to apply attributes on the wrapper.

```rust
use axum_typed_multipart::FieldData;
use serde::Serialize;
use tempfile::NamedTempFile;
use validy::core::Validate;

#[derive(Debug, Validate, Serialize)]
#[validate(asynchronous, payload, axum, multipart)]
#[wrapper_derive(Debug, Serialize)]
#[wrapper_attribute(try_from_multipart(strict))]
pub struct TestDTO {
	#[serde(skip)]
	#[wrapper_attribute(serde(skip))]
	#[wrapper_attribute(form_data(limit = "10MB"))]
	pub file: FieldData<NamedTempFile>,

	#[wrapper_attribute(form_data(field_name = "user_name"))]
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))]
	pub name: String,
	//...
}
```

## 📐 Useful Macros

Sometimes, you might prefer to use macros to declare errors or assertions.

### For `error` handling
  
All require the `macro_rules` feature flag to be enabled.

```rust
use validy::validation_error;
// SimpleValidationError
let error = validation_error!("field", "custom_code", "custom message");
```

```rust
use validy::validation_error;
// SimpleValidationError
let error = validation_error!("field", "custom_code");
```

```rust
use validy::validation_errors;
// ValidationErrors
let errors = validation_errors! {
  "a" => ("custom_code", "custom message"),
	"b" => ("nested", validation_errors! {
	  "c" => ("custom_code", "custom message"),
		"d" => [
		  ("custom_code", "custom message"), 
			("custom_code_2", "custom_message_2")
		]
	})
};
```

```rust
use validy::{nested_validation_error, validation_errors};
// NestedValidationError
let error = nested_validation_error!(
	"field",
	"custom_code",
	validation_errors! {
    "a" => ("custom_code", "custom message"),
	}
);
```

### For `test` assertions
  
All require the `macro_rules_assertions` feature flag to be enabled. They all require the input to implement `Debug` and/or `PartialEq` trait.

```rust
use validy::{
	assert_errors, assert_modification, assert_parsed, assert_validation,
	core::{Validate, ValidateAndModificate, ValidateAndParse},
};

#[derive(Debug, PartialEq, Clone, Default, Validate)]
struct Test {
	#[validate(range(1..=2, "out of range"))]
	a: u32,
}

let mut test = Test::default();
let result = Test::validate_and_parse(test.clone());

assert_errors!(result, test, {
  "a" => ("range", "out of range"),
});

test.a = 1;
let result = test.validate_and_modificate();
assert_validation!(result, test);
assert_modification!(test.a, 1, test);

let result = Test::validate_and_parse(test.clone());
assert_parsed!(result, test, Test { a: 1 });
```

## 💝 Complete Example

```rust
use axum::{Json, extract::State, http::StatusCode, response::{Response, IntoResponse}};
use validy::core::{Validate, ValidationError};
use std::{sync::Arc, fmt::Debug};

#[derive(Debug, Validate)]
#[validate(asynchronous, context = Arc<dyn UserService>, payload, axum)]
pub struct CreateUserExampleDTO {
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))] // Just changes the 'required' message.
	pub name: String,

	#[modificate(trim)]
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
	#[modificate(lowercase)] // You can modificate or validate it as a String, if it has a value.
	#[parse(inline(|x: String| x.parse::<u16>().unwrap_or(0)))] // You can then parse it to the final value type.
	#[modificate(inline(|x: &mut u16| *x = 3))] // And validate or modificate it again.
	#[validate(range(3..=12))] 
	pub dependent_id: u16,

	#[modificate(trim)]
	#[validate(length(0..=254, "tag must not be more than 254 characters"))]
	#[modificate(snake_case)]
	#[modificate(custom(modificate_tag))]
	pub tag: Option<String>, // 'tag' is truly optional.
	
	#[special(from_type(RoleWrapper))] // Required to correctly define the wrapper field type.
	#[special(nested(Role, RoleWrapper))] // Required to correctly validate nested content.
	// The wrapper type and the 'from_type' rule can be ignored when 'payload' is disabled.
	//#[special(nested(Role))]
	pub role: Option<Role>, // Can be optional or required.
	//pub role: Role,
}

// To use a struct in nested validations, it needs to derive 'Default'.
#[derive(Debug, Default, Validate)]
#[validate(payload, axum)]
pub struct Role {
	#[special(from_type(Vec<String>))]
	#[validate(length(1..=2))]
	#[special(for_each( // You can validate or modificate each item in a collection.
 	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
    parse(inline(|x: String| x.parse::<u32>().unwrap_or(0))), // Just another parse example.
    validate(inline(|x: &u32| *x > 1)), // Just a validation example.
 	  modificate(inline(|x: &mut u32| *x += 1))
	))]
	pub permissions: Vec<u32>,
	
	#[special(from_type(Vec<String>))]
	#[special(for_each(
	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
		parse(inline(|x: String| x.parse::<u32>().unwrap_or(0))),
	  validate(inline(|x: &u32| *x > 1)),
		modificate(inline(|x: &mut u32| *x += 1))
	))]
	pub alt_permissions: Vec<u32>,
}

// As a rule, the input for custom functions is '(&field, &field_name)'.
// All custom modification rules can also throw validation errors.
// Unfortunately, each modification has to return a new value instead of changing the existing one in-place.
// This ensures that changes are only committed at the end of the validation process.
fn modificate_tag(tag: &mut String, _field_name: &str) -> Result<(), ValidationError> {
	*tag = "new_tag".to_string();
	Ok(())
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

pub async fn create_user(
	State(service): State<Arc<dyn UserService>>,
	body: CreateUserExampleDTO,
) -> Result<Response, Response> {
	service.create(body.name, body.email, body.password).await?;
	Ok(StatusCode::CREATED.into_response())
}

#[async_trait::async_trait]
pub trait UserService: Send + Sync + Debug {
  async fn create(&self, name: String, email: String, password: String) -> Result<(), Response>;
	async fn email_exists(&self, email: &str) -> Result<bool, Response>;
	//...
}
```

## 📁 More Examples

If the examples aren't enough, I've included a more complete and documented example [here](/docs/complete_example.md). You also can use the [tests](/tests) as a reference.

## 🎯 Work In Progress

- [ ] Failure mode test coverage.
  - The current default is `FailOncePerField` (covered by the tests).
- [ ] Parse ips.

## 🎁 For Developers

You can run all tests with `cargo test-all`. To see the generated code from the `derive` macros, you can run the `expand.sh` script (this requires `cargo expand`). It will compile, generate, and check all tests.

> This is a personal project maintained by a graduate student. Maintenance may be limited, but I do my best to keep it in good shape.
