use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::attributes::ValidationAttributes;

pub fn get_async_modification_axum_extension(struct_name: &Ident, attributes: &ValidationAttributes) -> TokenStream {
	if !cfg!(feature = "axum") || !attributes.axum {
		quote! {}
	} else {
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
           	#struct_name: AsyncValidateAndModificate,
        {
         	type Rejection = Response;

         	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        		let Json(mut object): Json<#struct_name> = Json::from_request(req, state).await.map_err(|e| e.into_response())?;

        		match object.async_validate_and_modificate().await {
         			Ok(_) => Ok(object),
         			Err(errors) => Err((StatusCode::BAD_REQUEST, Json(errors)).into_response()),
        		}
         	}
        }
  		};
  	};

		result
	}
}

pub fn get_async_modification_with_context_axum_extension(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
) -> TokenStream {
	if !cfg!(feature = "axum") || !attributes.axum {
		quote! {}
	} else {
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
           	#struct_name: SpecificAsyncValidateAndModificateWithContext,
           	<#struct_name as SpecificAsyncValidateAndModificateWithContext>::Context: FromRef<S>,
        {
         	type Rejection = Response;

         	async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        		let Json(mut object): Json<#struct_name> = Json::from_request(req, state).await.map_err(|e| e.into_response())?;

        		let context: <UserDTO as SpecificAsyncValidateAndModificateWithContext>::Context = FromRef::from_ref(state);

        		match object.specific_async_validate_and_modificate_with_context(&context).await {
         			Ok(_) => Ok(object),
         			Err(errors) => Err((StatusCode::BAD_REQUEST, Json(errors)).into_response()),
        		}
         	}
        }
     	};
    };

		result
	}
}
