use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn get_payload_factory_boilerplates(struct_name: &Ident, wrapper_ident: &Ident) -> TokenStream {
	let method = quote! { <#struct_name as ValidateAndParse<#wrapper_ident>>::validate_and_parse(wrapper) };
	let boilerplates = vec![
		get_payload_with_context_boilerplate(struct_name, wrapper_ident, None, &method),
		get_async_payload_boilerplate(struct_name, wrapper_ident, &method),
		get_async_payload_with_context_boilerplate(struct_name, wrapper_ident, None, &method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_payload_with_context_factory_boilerplates(
	struct_name: &Ident,
	wrapper_ident: &Ident,
	context_type: &Type,
) -> TokenStream {
	let method = quote! { <#struct_name as ValidateAndParseWithContext<#wrapper_ident, #context_type>>::validate_and_parse_with_context(wrapper, context) };
	let boilerplates = vec![get_async_payload_with_context_boilerplate(
		struct_name,
		wrapper_ident,
		Some(context_type),
		&method,
	)];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_async_payload_factory_boilerplates(struct_name: &Ident, wrapper_ident: &Ident) -> TokenStream {
	let method =
		quote! { <#struct_name as AsyncValidateAndParse<#wrapper_ident>>::async_validate_and_parse(wrapper).await };
	let boilerplates = vec![get_async_payload_with_context_boilerplate(
		struct_name,
		wrapper_ident,
		None,
		&method,
	)];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_payload_with_context_boilerplate(
	struct_name: &Ident,
	wrapper_ident: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
   	  impl ValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
       	fn validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
  			  #method
  		  }
  	  }

     impl SpecificValidateAndParseWithContext for #struct_name {
        type Wrapper = #wrapper_ident;
        type Context = #context_type;
        fn specific_validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
          #method
        }
  	  }
    },
    None => quote! {
  	  impl<C> ValidateAndParseWithContext<#wrapper_ident, C> for #struct_name {
  			fn validate_and_parse_with_context(mut wrapper: #wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
  			  #method
  		  }
  	  }
    }
	};

	result
}

pub fn get_payload_boilerplate(struct_name: &Ident, wrapper_ident: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
    impl ValidateAndParse<#wrapper_ident> for #struct_name {
      fn validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
       	#method
      }
    }

    impl SpecificValidateAndParse for #struct_name {
      type Wrapper = #wrapper_ident;
      fn specific_validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
       	#method
      }
    }
	};

	result
}

pub fn get_async_payload_boilerplate(struct_name: &Ident, wrapper_ident: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
    #[async_trait]
    impl AsyncValidateAndParse<#wrapper_ident> for #struct_name {
      async fn async_validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
       	#method
      }
    }

    #[async_trait]
    impl SpecificAsyncValidateAndParse for #struct_name {
      type Wrapper = #wrapper_ident;
      async fn specific_async_validate_and_parse(mut wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
       	#method
      }
    }
	};

	result
}

pub fn get_async_payload_with_context_boilerplate(
	struct_name: &Ident,
	wrapper_ident: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
      #[async_trait]
     	impl AsyncValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
     	  async fn async_validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
         	#method
     		}
      }

     	#[async_trait]
     	impl SpecificAsyncValidateAndParseWithContext for #struct_name {
        type Wrapper = #wrapper_ident;
        type Context = #context_type;
     	  async fn specific_async_validate_and_parse_with_context(mut wrapper: #wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
         	#method
     		}
      }
    },
    None => quote! {
     	#[async_trait]
     	impl<C> AsyncValidateAndParseWithContext<#wrapper_ident, C> for #struct_name {
     	  async fn async_validate_and_parse_with_context(mut wrapper: #wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
         	#method
     		}
      }
    }
	};

	result
}
