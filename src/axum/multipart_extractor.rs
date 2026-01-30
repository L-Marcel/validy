use axum::{
	Json,
	extract::{FromRef, FromRequest, Multipart, Request},
	response::{IntoResponse, Response},
};
use axum_typed_multipart::TryFromMultipartWithState;

use crate::{
	axum::valid::ValidMultipart, core::SpecificAsyncValidateAndParseWithContext, settings::ValidationSettings,
};

impl<S, T: SpecificAsyncValidateAndParseWithContext> FromRequest<S> for ValidMultipart<T>
where
	S: Send + Sync,
	T::Context: FromRef<S>,
	T::Wrapper: Send + Sync + TryFromMultipartWithState<S>,
{
	type Rejection = Response;

	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
		let mut multipart = Multipart::from_request(req, state)
			.await
			.map_err(|e| e.into_response())?;

		let wrapper = T::Wrapper::try_from_multipart_with_state(&mut multipart, state)
			.await
			.map_err(|e| e.into_response())?;

		let context: T::Context = FromRef::from_ref(state);

		match T::specific_async_validate_and_parse_with_context(wrapper, &context).await {
			Ok(object) => Ok(ValidMultipart(object)),
			Err(errors) => Err((ValidationSettings::get_failure_multipart_status_code(), Json(errors)).into_response()),
		}
	}
}
