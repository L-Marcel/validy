use axum::{
	Json, Router,
	body::Body,
	http::{Method, Request, StatusCode, header},
	response::IntoResponse,
	routing::post,
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;

use serde::{Deserialize, Serialize};
use validy::core::{Validate, ValidationError};

use crate::axum::mocks::{ImplMockedService, get_state};

#[derive(Debug, Deserialize, Serialize, Validate)]
#[validate(asynchronous, payload, axum)]
pub struct TestDTO {
	#[modify(trim)]
	#[validate(length(3..=120, "name must be between 3 and 120 characters"))]
	#[validate(required("name is required"))]
	pub name: String,

	#[modify(trim)]
	#[validate(email("invalid email format", "bad_format"))]
	#[validate(async_custom(validate_unique_email))]
	#[validate(inline(|_| true))]
	#[validate(length(0..=254, "email must not be more than 254 characters"))]
	pub email: String,

	#[validate(length(3..=12, code = "size", message = "password must be between 3 and 12 characters"))]
	pub password: String,

	#[special(from_type(String))]
	#[modify(lowercase)]
	#[modify(inline(|_| 3))]
	#[validate(range(3..=12))]
	pub dependent_id: u16,

	#[modify(trim)]
	#[validate(length(0..=254, "tag must not be more than 254 characters"))]
	#[modify(snake_case)]
	#[modify(custom(modify_tag))]
	pub tag: Option<String>,

	#[special(from_type(RoleDTOWrapper))]
	#[special(nested(RoleDTO, RoleDTOWrapper))]
	pub role: Option<RoleDTO>,
}

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
#[validate(payload, axum)]
pub struct RoleDTO {
	#[special(from_type(Vec<String>))]
	#[validate(length(1..=2))]
	#[special(for_each(
 	  config(from_item = String, from_collection = Vec<String>, to_collection = Vec<u32>),
    modify(inline(|x: &str| ::serde_json::from_str::<u32>(x).unwrap_or(0))),
    validate(inline(|x: &u32| *x > 1)),
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

fn modify_tag(tag: &str, _field_name: &str) -> (String, Option<ValidationError>) {
	(tag.to_string() + "_modified", None)
}

async fn validate_unique_email(email: &str, field_name: &str) -> Result<(), ValidationError> {
	if email == "test@gmail.com" {
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

pub async fn test_handle(data: TestDTO) -> Result<impl IntoResponse, (StatusCode, String)> {
	Ok((StatusCode::CREATED, Json(data)))
}

pub async fn test_two_handle(data: RoleDTO) -> Result<impl IntoResponse, (StatusCode, String)> {
	Ok((StatusCode::CREATED, Json(data)))
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
			json!({
				"name": "  Alice  ",
				"email": "alice@test.com",
				"password": "secure",
				"dependent_id": "99",
			}),
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
			json!({
				"name": "Bob",
				"email": "bob@test.com",
				"password": "secure",
				"dependent_id": "10",
				"tag": "  My Super Tag  ",
				"role": {
					"permissions": ["2", "10"],
					"alt_permissions": ["2"]
				}
			}),
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
			json!({
				"name": "Charlie",
				"email": "test@gmail.com",
				"password": "ab",
				"dependent_id": "5"
			}),
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
			json!({
				"name": "Dave",
				"email": "dave@test.com",
				"password": "secure",
				"dependent_id": "5",
				"role": {
					"permissions": [],
					"alt_permissions": ["2"]
				}
			}),
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
			json!({
				"permissions": ["2", "10"],
				"alt_permissions": ["2"]
			}),
			json!({
				"permissions": [3, 11],
				"alt_permissions": [3]
			}),
		),
		(
			"/test_two",
			StatusCode::BAD_REQUEST,
			json!({
			  "permissions": ["0"],
				"alt_permissions": ["2"]
			}),
			json!({
			  "permissions": [{
					"code": "inline",
					"message": "invalid"
			  }]
			}),
		),
	];

	for (route, expected_status, case, expected) in cases.iter() {
		let req = Request::builder()
			.method(Method::POST)
			.uri(*route)
			.header(header::CONTENT_TYPE, "application/json")
			.body(Body::from(case.to_string()))
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
				case, expected_status, status, error_msg
			);
		}

		let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

		assert_eq!(
			&body_json, expected,
			"Result did not match expectations for {:#?}.",
			case
		);
	}
}
