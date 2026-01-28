use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn get_async_payload_axum_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
 	  const _: () = {
 			use serde::de::DeserializeOwned;
 			use axum::{
				Json,
				extract::{FromRef, FromRequest, Request},
				http::StatusCode,
				response::{IntoResponse, Response},
 	    };

   	  impl<S> FromRequest<S> for #struct_name
   			where
   			  S: Send + Sync,
   			  #struct_name: SpecificAsyncValidateAndParse,
   			  <#struct_name as SpecificAsyncValidateAndParse>::Wrapper: DeserializeOwned + Send + Sync,
   	  {
				type Rejection = Response;

				async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
 					let Json(mut wrapper) = Json::<<#struct_name as SpecificAsyncValidateAndParse>::Wrapper>::from_request(req, state)
						.await
						.map_err(|e| e.into_response())?;

 					match #struct_name::specific_async_validate_and_parse(wrapper).await {
						Ok(object) => Ok(object),
						Err(errors) => Err((ValidationSettings::get_failure_status_code(), Json(errors)).into_response()),
 					}
				}
 	    }
		};
 	};

	result
}

pub fn get_async_payload_with_context_axum_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
    const _: () = {
  		use serde::de::DeserializeOwned;
  		use axum::{
   			Json,
   			extract::{FromRef, FromRequest, Request},
   			http::StatusCode,
   			response::{IntoResponse, Response},
      };

  		impl<S> FromRequest<S> for #struct_name
  		  where
  				S: Send + Sync,
  				#struct_name: SpecificAsyncValidateAndParseWithContext,
  				<#struct_name as SpecificAsyncValidateAndParseWithContext>::Context: FromRef<S>,
  				<#struct_name as SpecificAsyncValidateAndParseWithContext>::Wrapper: DeserializeOwned + Send + Sync,
      {
       	type Rejection = Response;

 			  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
  				let Json(mut wrapper): Json<<#struct_name as SpecificAsyncValidateAndParseWithContext>::Wrapper> =
 					Json::from_request(req, state).await.map_err(|e| e.into_response())?;

  				let context: <#struct_name as SpecificAsyncValidateAndParseWithContext>::Context = FromRef::from_ref(state);

  				match #struct_name::specific_async_validate_and_parse_with_context(wrapper, &context).await {
   					Ok(object) => Ok(object),
   					Err(errors) => Err((ValidationSettings::get_failure_status_code(), Json(errors)).into_response()),
  				}
 			  }
      }
   	};
  };

	result
}

pub fn get_async_payload_axum_multipart_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
 	  const _: () = {
      use axum_typed_multipart::{TryFromMultipartWithState};
  		use axum::{
   			Json,
   			extract::{FromRequest, Request, Multipart, State},
   			http::StatusCode,
   			response::{IntoResponse, Response},
      };

   	  impl<S> FromRequest<S> for #struct_name
   			where
   			  S: Send + Sync,
   			  #struct_name: SpecificAsyncValidateAndParse,
   			  <#struct_name as SpecificAsyncValidateAndParse>::Wrapper: Send + Sync + TryFromMultipartWithState<S>,
   	  {
				type Rejection = Response;

				async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
				  let mut multipart = Multipart::from_request(req, state)
       			.await
       			.map_err(|e| e.into_response())?;

					let mut wrapper = <#struct_name as SpecificAsyncValidateAndParse>::Wrapper::try_from_multipart_with_state(
       			&mut multipart,
       			state,
      		).await.map_err(|e| e.into_response())?;

 					match #struct_name::specific_async_validate_and_parse(wrapper).await {
						Ok(object) => Ok(object),
						Err(errors) => Err((ValidationSettings::get_failure_multipart_status_code(), Json(errors)).into_response()),
 					}
				}
 	    }
		};
 	};

	result
}

pub fn get_async_payload_with_context_axum_multipart_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
    const _: () = {
      use axum_typed_multipart::{TryFromMultipartWithState};
  		use axum::{
   			Json,
   			extract::{FromRef, FromRequest, Request, Multipart, State},
   			http::StatusCode,
   			response::{IntoResponse, Response},
      };

      impl<S> FromRequest<S> for #struct_name
        where
         	S: Send + Sync,
         	#struct_name: SpecificAsyncValidateAndParseWithContext,
         	<#struct_name as SpecificAsyncValidateAndParseWithContext>::Context: FromRef<S>,
         	<#struct_name as SpecificAsyncValidateAndParseWithContext>::Wrapper: Send + Sync + TryFromMultipartWithState<S>,
      {
       	type Rejection = Response;

       	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
      		let mut multipart = Multipart::from_request(req, state)
       			.await
       			.map_err(|e| e.into_response())?;

      		let mut wrapper = <#struct_name as SpecificAsyncValidateAndParseWithContext>::Wrapper::try_from_multipart_with_state(
       			&mut multipart,
       			state,
      		).await.map_err(|e| e.into_response())?;

          let context: <#struct_name as SpecificAsyncValidateAndParseWithContext>::Context = FromRef::from_ref(state);

      		match #struct_name::specific_async_validate_and_parse_with_context(wrapper, &context).await {
      		  Ok(object) => Ok(object),
       			Err(errors) => Err((ValidationSettings::get_failure_multipart_status_code(), Json(errors)).into_response()),
      		}
     	  }
      }
    };
  };

	result
}
