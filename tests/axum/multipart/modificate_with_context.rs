use async_trait::async_trait;
use axum::extract::multipart::Field;
use axum::{
	Json, Router,
	body::Body,
	extract::State,
	http::{Method, Request, StatusCode, header},
	response::IntoResponse,
	routing::post,
};
use axum_typed_multipart::{FieldData, TryFromField, TryFromMultipart, TypedMultipartError};

use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use tempfile::NamedTempFile;
use tower::ServiceExt;
use validy::core::{Validate, ValidationError};

use crate::axum::mocks::{ImplMockedService, MockedService, get_state};

#[derive(Debug, TryFromMultipart, Validate, Serialize)]
#[validate(context = Arc<dyn MockedService>, modificate, axum, multipart)]
pub struct TestDTO {
	#[special(ignore)]
	#[serde(skip)]
	pub file: FieldData<NamedTempFile>,

	#[form_data(field_name = "user_name")]
	#[modificate(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	pub name: String,

	#[modificate(trim)]
	#[validate(email("invalid email format", "bad_format"))]
	#[validate(custom_with_context(validate_unique_email))]
	#[validate(inline(|_| true))]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	pub email: String,

	#[form_data(limit = "20B")]
	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,

	#[modificate(inline(|x: &mut u16| *x = 3))]
	#[validate(range(3..=12))]
	pub dependent_id: u16,

	#[modificate(trim)]
	#[validate(length(0..=254, "tag must not be more than 254 characters"))]
	#[modificate(snake_case)]
	#[modificate(custom(modificate_tag))]
	pub tag: Option<String>,

	#[special(nested(RoleDTO))]
	pub role: Option<RoleDTO>,
}

#[derive(Debug, Clone, Deserialize, TryFromMultipart, Serialize, Default, Validate)]
#[validate(modificate, axum, multipart)]
pub struct RoleDTO {
	#[validate(length(1..=2))]
	#[special(for_each(
 	  config(from_item = u32, from_collection = Vec<u32>, to_collection = Vec<u32>),
    validate(inline(|x: &u32| *x > 1)),
 	  modificate(inline(|x: &mut u32| *x += 1))
	))]
	pub permissions: Vec<u32>,

	#[special(for_each(
	  config(from_item = u32, from_collection = Vec<u32>, to_collection = Vec<u32>),
	  validate(inline(|x: &u32| *x > 1)),
		modificate(inline(|x: &mut u32| *x += 1))
	))]
	pub alt_permissions: Vec<u32>,
}

#[async_trait]
impl TryFromField for RoleDTO {
	async fn try_from_field(field: Field<'_>, _limit_bytes: Option<usize>) -> Result<Self, TypedMultipartError> {
		let name = field.name().unwrap_or_default().to_string();
		let bytes = field.bytes().await?;

		// WARNING: No manual size limit check implemented here.
		// SECURITY: Manual size limit handling is required for TryFromFieldWithState.
		// Unlike TryFromField which can leverage TryFromChunks for automatic size checking,
		// the stateful variant requires explicit implementation.

		let json_str = std::str::from_utf8(&bytes).map_err(|e| TypedMultipartError::WrongFieldType {
			field_name: name.clone(),
			source: e.into(),
			wanted_type: "String".to_string(),
		})?;

		serde_json::from_str(json_str).map_err(|e| TypedMultipartError::WrongFieldType {
			field_name: name,
			source: e.into(),
			wanted_type: "RoleDTO".to_string(),
		})
	}
}

fn modificate_tag(tag: &mut String, _field_name: &str) -> Result<(), ValidationError> {
	*tag = (tag.to_string() + "_modified").to_string();
	Ok(())
}

fn validate_unique_email(
	email: &str,
	field_name: &str,
	service: &Arc<dyn MockedService>,
) -> Result<(), ValidationError> {
	let result = service.sync_email_exists(email);

	if result {
		Err(ValidationError::builder()
			.with_field(field_name.to_string())
			.as_simple("unique")
			.with_message("e-mail must be unique")
			.build()
			.into())
	} else {
		Ok(())
	}
}

pub async fn test_handle(
	State(_): State<Arc<dyn MockedService>>,
	data: TestDTO,
) -> Result<impl IntoResponse, (StatusCode, String)> {
	Ok((StatusCode::CREATED, Json(data)))
}

pub async fn test_two_handle(data: RoleDTO) -> Result<impl IntoResponse, (StatusCode, String)> {
	Ok((StatusCode::CREATED, Json(data)))
}

fn build_multipart_body(fields: &[(&str, &str)]) -> (String, Body) {
	let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
	let mut body = String::new();
	for (name, value) in fields {
		body.push_str(&format!("--{}\r\n", boundary));
		body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name));
		body.push_str(value);
		body.push_str("\r\n");
	}
	body.push_str(&format!("--{}--\r\n", boundary));

	let content_type = format!("multipart/form-data; boundary={}", boundary);
	(content_type, Body::from(body))
}

#[tokio::test]
async fn should_validate_requests() {
	let service = Arc::new(ImplMockedService {});
	let state = get_state(service).await;

	let app = Router::new()
		.route("/test", post(test_handle))
		.route("/test_two", post(test_two_handle))
		.with_state(state);

	let cases = [
		(
			"/test",
			StatusCode::CREATED,
			vec![
				("user_name", "  Alice  "),
				("email", "alice@test.com"),
				("password", "secure"),
				("dependent_id", "12"),
				("file", "empty file"),
			],
			json!({
				"name": "Alice",
				"email": "alice@test.com",
				"password": "secure",
				"dependent_id": 3,
				"tag": null,
				"role": null
			}),
		),
		(
			"/test",
			StatusCode::CREATED,
			vec![
				("user_name", "Bob"),
				("email", "bob@test.com"),
				("password", "secure"),
				("dependent_id", "10"),
				("tag", "  My Super Tag  "),
				("role", r#"{"permissions": [2, 10], "alt_permissions": [2]}"#),
				("file", "empty file"),
			],
			json!({
				"name": "Bob",
				"email": "bob@test.com",
				"password": "secure",
				"dependent_id": 3,
				"tag": "my_super_tag_modified",
				"role": {
					"permissions": [3, 11],
					"alt_permissions": [3]
				}
			}),
		),
		(
			"/test",
			StatusCode::BAD_REQUEST,
			vec![
				("user_name", "Charlie"),
				("email", "test@gmail.com"),
				("password", "ab"),
				("dependent_id", "5"),
				("file", "empty file"),
			],
			json!({
				"email": [{
					"code": "unique",
					"message": "e-mail must be unique"
				}],
				"password": [{
					"code": "size",
					"message": "password must be between 3 and 12 characters"
				}]
			}),
		),
		(
			"/test",
			StatusCode::BAD_REQUEST,
			vec![
				("user_name", "Dave"),
				("email", "dave@test.com"),
				("password", "secure"),
				("dependent_id", "5"),
				("role", r#"{"permissions": [], "alt_permissions": [2]}"#),
				("file", "empty file"),
			],
			json!({
				"role": [{
					"code": "nested",
					"errors": {
						"permissions": [{
							"code": "length",
							"message": "length out of range"
						}]
					}
				}]
			}),
		),
		(
			"/test_two",
			StatusCode::CREATED,
			vec![("permissions", "2"), ("permissions", "10"), ("alt_permissions", "2")],
			json!({
				"permissions": [3, 11],
				"alt_permissions": [3]
			}),
		),
		(
			"/test_two",
			StatusCode::BAD_REQUEST,
			vec![("permissions", "0"), ("alt_permissions", "2")],
			json!({
				"permissions": [{
					"code": "inline",
					"message": "invalid"
				}]
			}),
		),
		(
			"/test",
			StatusCode::BAD_REQUEST,
			vec![
				("user_name", "  Alice  "),
				("email", "alice@test.com"),
				("file", "empty file"),
			],
			json!("field 'password' is required"),
		),
		(
			"/test",
			StatusCode::PAYLOAD_TOO_LARGE,
			vec![
				("user_name", "A valid name"),
				("email", "valid@email.com"),
				("user_name", "Bob"),
				("email", "bob@test.com"),
				("password", "this field is definitely way too long for 20 bytes limit"),
				("dependent_id", "10"),
				("tag", "  My Super Tag  "),
				("role", r#"{"permissions": [2, 10], "alt_permissions": [2]}"#),
				("file", "empty file"),
			],
			json!("field 'password' is larger than 20 bytes"),
		),
	];

	for (route, expected_status, form_data, expected) in cases.iter() {
		let (content_type, body) = build_multipart_body(form_data);

		let req = Request::builder()
			.method(Method::POST)
			.uri(*route)
			.header(header::CONTENT_TYPE, content_type)
			.body(body)
			.expect("should create a request");

		let response = app.clone().oneshot(req).await.expect("should execute");

		let status = response.status();

		let body_bytes = match response.into_body().collect().await {
			Ok(body) => body.to_bytes(),
			Err(error) => panic!("Can't parse the resut body: {}", error),
		};

		if status != *expected_status {
			let error_msg = String::from_utf8_lossy(&body_bytes);

			panic!(
				"Result did not match expectations for {:#?}. Expected status {}, received {} and {:#?}",
				form_data, expected_status, status, error_msg
			);
		}

		let body_json: Value = serde_json::from_slice(&body_bytes)
			.unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&body_bytes).to_string()));

		assert_eq!(
			&body_json, expected,
			"Result did not match expectations for {:#?}.",
			form_data
		);
	}
}
