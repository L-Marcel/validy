#[macro_export]
macro_rules! validation_errors {
  ( $( $key:expr => ($code:expr, $val:expr) ),* $(,)? ) => {
    {
      use std::borrow::Cow;
      use $crate::core::IntoValidationError;
      let mut m = $crate::core::ValidationErrors::new();
      $(
          let field = Cow::from($key);
          let code = Cow::from($code);
          let error = $val.into_error(field, code);
          m.insert($key.into(), error);
      )*
      m
    }
  };
}

#[macro_export]
macro_rules! validation_error {
	($field:expr, $code:expr, $message:expr) => {
		$crate::core::ValidationError::Leaf($crate::core::SimpleValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			message: Some(std::borrow::Cow::from($message)),
		})
	};
	($field:expr, $code:expr) => {
		$crate::core::ValidationError::Leaf($crate::SimpleValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			message: None,
		})
	};
}

#[macro_export]
macro_rules! nested_validation_error {
	($field:expr, $code:expr, $errors:expr) => {
		$crate::core::ValidationError::Node($crate::NestedValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			errors: $errors,
		})
	};
}
