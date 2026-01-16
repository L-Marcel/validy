use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::attributes::ValidationAttributes;

pub fn get_async_payload_axum_extension(struct_name: &Ident, attributes: &ValidationAttributes) -> TokenStream {
	if !cfg!(feature = "axum") || !attributes.axum {
		quote! {}
	} else {
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
  					let Json(wrapper) = Json::<<#struct_name as SpecificAsyncValidateAndParse>::Wrapper>::from_request(req, state)
  						.await
  						.map_err(|e| e.into_response())?;

  					match #struct_name::specific_async_validate_and_parse(&wrapper).await {
  						Ok(object) => Ok(object),
  						Err(errors) => Err((StatusCode::BAD_REQUEST, Json(errors)).into_response()),
  					}
  				}
  	    }
  		};
  	};

		result
	}
}

pub fn get_async_payload_with_context_axum_extension(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
) -> TokenStream {
	if !cfg!(feature = "axum") || !attributes.axum {
		quote! {}
	} else {
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
    				let Json(wrapper): Json<<#struct_name as SpecificAsyncValidateAndParseWithContext>::Wrapper> =
   					Json::from_request(req, state).await.map_err(|e| e.into_response())?;

    				let context: <#struct_name as SpecificAsyncValidateAndParseWithContext>::Context = FromRef::from_ref(state);

    				match #struct_name::specific_async_validate_and_parse_with_context(&wrapper, &context).await {
     					Ok(object) => Ok(object),
     					Err(errors) => Err((StatusCode::BAD_REQUEST, Json(errors)).into_response()),
    				}
   			  }
        }
     	};
    };

		result
	}
}
