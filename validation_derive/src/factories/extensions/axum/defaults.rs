use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn get_async_default_axum_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
 	  const _: () = {
 			use axum::{
				Json,
				extract::{FromRef, FromRequest, Request},
				http::StatusCode,
				response::{IntoResponse, Response},
 	    };

      impl<S> FromRequest<S> for #struct_name
        where
         	S: Send + Sync,
         	#struct_name: AsyncValidate,
      {
       	type Rejection = Response;

       	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
      		let Json(object): Json<#struct_name> = Json::from_request(req, state).await.map_err(|e| e.into_response())?;

      		match object.async_validate().await {
       			Ok(_) => Ok(object),
       			Err(errors) => Err((ValidationSettings::get_failure_status_code(), Json(errors)).into_response()),
      		}
       	}
      }
		};
 	};

	result
}

pub fn get_async_default_with_context_axum_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
    const _: () = {
  		use axum::{
   			Json,
   			extract::{FromRef, FromRequest, Request},
   			http::StatusCode,
   			response::{IntoResponse, Response},
      };

      impl<S> FromRequest<S> for #struct_name
        where
         	S: Send + Sync,
         	#struct_name: SpecificAsyncValidateWithContext,
         	<#struct_name as SpecificAsyncValidateWithContext>::Context: FromRef<S>,
      {
       	type Rejection = Response;

       	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
      		let Json(object): Json<#struct_name> = Json::from_request(req, state).await.map_err(|e| e.into_response())?;

      		let context: <#struct_name as SpecificAsyncValidateWithContext>::Context = FromRef::from_ref(state);

      		match object.specific_async_validate_with_context(&context).await {
       			Ok(_) => Ok(object),
       			Err(errors) => Err((ValidationSettings::get_failure_status_code(), Json(errors)).into_response()),
      		}
       	}
      }
   	};
  };

	result
}

pub fn get_async_default_axum_multipart_extension(struct_name: &Ident) -> TokenStream {
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
         	#struct_name: AsyncValidate,
      {
       	type Rejection = Response;

       	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
      		let mut multipart = Multipart::from_request(req, state)
       			.await
       			.map_err(|e| e.into_response())?;

      		let object = #struct_name::try_from_multipart_with_state(&mut multipart, state)
       			.await
       			.map_err(|e| e.into_response())?;

      		match object.async_validate().await {
       			Ok(_) => Ok(object),
       			Err(errors) => Err((ValidationSettings::get_failure_multipart_status_code(), Json(errors)).into_response()),
      		}
       	}
      }
    };
  };

	result
}

pub fn get_async_default_with_context_axum_multipart_extension(struct_name: &Ident) -> TokenStream {
	#[rustfmt::skip]
  let result = quote! {
    const _: () = {
      use axum::{
   			Json,
   			extract::{FromRef, FromRequest, Request},
   			http::StatusCode,
   			response::{IntoResponse, Response},
      };

      impl<S> FromRequest<S> for #struct_name
        where
         	S: Send + Sync,
         	#struct_name: SpecificAsyncValidateWithContext,
         	<#struct_name as SpecificAsyncValidateWithContext>::Context: FromRef<S>,
      {
        type Rejection = Response;

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
       		let mut multipart = Multipart::from_request(req, state)
        		.await
        		.map_err(|e| e.into_response())?;

      		let object = #struct_name::try_from_multipart_with_state(&mut multipart, state)
       			.await
        		.map_err(|e| e.into_response())?;

       		let context: <#struct_name as SpecificAsyncValidateWithContext>::Context = FromRef::from_ref(state);

       		match object.specific_async_validate_with_context(&context).await {
       			Ok(_) => Ok(object),
       			Err(errors) => Err((ValidationSettings::get_failure_multipart_status_code(), Json(errors)).into_response()),
       		}
        }
      }
    };
  };

	result
}
