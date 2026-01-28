use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::factories::boilerplates::payloads::{
	get_async_payload_boilerplate, get_async_payload_with_context_boilerplate, get_payload_boilerplate,
	get_payload_with_context_boilerplate,
};

pub fn get_modification_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.validate_and_modificate() };
	let payload_method = quote! {
	  wrapper.validate_and_modificate()?;
		Ok(wrapper)
	};

	let boilerplates = vec![
		get_modification_with_context_boilerplate(struct_name, None, &method),
		get_async_modification_boilerplate(struct_name, &method),
		get_async_modification_with_context_boilerplate(struct_name, None, &method),
		get_payload_boilerplate(struct_name, struct_name, &payload_method),
		get_payload_with_context_boilerplate(struct_name, struct_name, None, &payload_method),
		get_async_payload_boilerplate(struct_name, struct_name, &payload_method),
		get_async_payload_with_context_boilerplate(struct_name, struct_name, None, &payload_method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_modification_with_context_factory_boilerplates(struct_name: &Ident, context_type: &Type) -> TokenStream {
	let method = quote! { self.validate_and_modificate_with_context(context) };
	let payload_method = quote! {
	  wrapper.validate_and_modificate_with_context(context)?;
		Ok(wrapper)
	};

	let boilerplates = vec![
		get_async_modification_with_context_boilerplate(struct_name, Some(context_type), &method),
		get_async_payload_with_context_boilerplate(struct_name, struct_name, Some(context_type), &payload_method),
		get_payload_with_context_boilerplate(struct_name, struct_name, Some(context_type), &payload_method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_async_modification_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.async_validate_and_modificate().await };
	let payload_method = quote! {
	  wrapper.async_validate_and_modificate().await?;
		Ok(wrapper)
	};

	let boilerplates = vec![
		get_async_modification_with_context_boilerplate(struct_name, None, &method),
		get_async_payload_with_context_boilerplate(struct_name, struct_name, None, &payload_method),
		get_async_payload_boilerplate(struct_name, struct_name, &payload_method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_async_modification_with_context_factory_boilerplates(
	struct_name: &Ident,
	context_type: &Type,
) -> TokenStream {
	let payload_method = quote! {
	  wrapper.async_validate_and_modificate_with_context(context).await?;
		Ok(wrapper)
	};

	let boilerplates = vec![get_async_payload_with_context_boilerplate(
		struct_name,
		struct_name,
		Some(context_type),
		&payload_method,
	)];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_modification_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
   	  impl ValidateAndModificateWithContext<#context_type> for #struct_name {
        fn validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }

			impl SpecificValidateAndModificateWithContext for #struct_name {
			  type Context = #context_type;
        fn specific_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
          #method
        }
      }
    },
    None => quote! {
      impl<C> ValidateAndModificateWithContext<C> for #struct_name {
			  fn validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}

pub fn get_modification_boilerplate(struct_name: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
		impl ValidateAndModificate for #struct_name {
		  fn validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
				#method
			}
	  }
	};

	result
}

pub fn get_async_modification_boilerplate(struct_name: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
		#[async_trait]
	  impl AsyncValidateAndModificate for #struct_name {
		  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
		    #method
		  }
	  }
	};

	result
}

pub fn get_async_modification_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
  		#[async_trait]
		  impl AsyncValidateAndModificateWithContext<#context_type> for #struct_name {
		    async fn async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
				  #method
			  }
		  }

  		#[async_trait]
		  impl SpecificAsyncValidateAndModificateWithContext for #struct_name {
				type Context = #context_type;
		    async fn specific_async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
				  #method
			  }
		  }
    },
    None => quote! {
      #[async_trait]
		  impl<C> AsyncValidateAndModificateWithContext<C> for #struct_name {
			  async fn async_validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}
