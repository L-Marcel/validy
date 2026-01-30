use axum::{
	Json,
	extract::{FromRef, FromRequest, Request},
	response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

use crate::{axum::valid::Valid, core::SpecificAsyncValidateAndParseWithContext, settings::ValidationSettings};

impl<S, T> FromRequest<S> for Valid<T>
where
	S: Send + Sync,
	T: SpecificAsyncValidateAndParseWithContext,
	T::Context: FromRef<S>,
	T::Wrapper: DeserializeOwned + Send + Sync,
{
	type Rejection = Response;

	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
		let Json(wrapper): Json<T::Wrapper> = Json::from_request(req, state).await.map_err(|e| e.into_response())?;

		let context: T::Context = FromRef::from_ref(state);

		match T::specific_async_validate_and_parse_with_context(wrapper, &context).await {
			Ok(object) => Ok(Valid(object)),
			Err(errors) => Err((ValidationSettings::get_failure_status_code(), Json(errors)).into_response()),
		}
	}
}
