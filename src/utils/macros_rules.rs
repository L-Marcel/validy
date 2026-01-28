#[macro_export]
macro_rules! validation_errors {
  ( $( $key:expr => $val:tt ),* $(,)? ) => {
    {
      use std::borrow::Cow;
      use ::validy::core::IntoValidationError;
      let mut m = ::validy::core::ValidationErrors::new();
      $(
        ::validy::validation_errors!(@insert m, $key, $val);
      )*
      m
    }
  };
  (@insert $m:ident, $key:expr, [ $( ($code:expr, $val:expr) ),* $(,)? ]) => {
    {
      let entry = $m.entry($key.into()).or_default();
      $(
        let field = Cow::from($key);
        let code = Cow::from($code);
        entry.push($val.into_error(field, code));
      )*
    }
  };
  (@insert $m:ident, $key:expr, ($code:expr, $val:expr)) => {
    {
      let field = Cow::from($key);
      let code = Cow::from($code);
      $m.entry($key.into()).or_default().push($val.into_error(field, code));
    }
  };
}

#[macro_export]
macro_rules! validation_error {
	($field:expr, $code:expr, $message:expr) => {
		::validy::core::ValidationError::Leaf(::validy::core::SimpleValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			message: Some(std::borrow::Cow::from($message)),
		})
	};
	($field:expr, $code:expr) => {
		::validy::core::ValidationError::Leaf(::validy::core::SimpleValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			message: None,
		})
	};
}

#[macro_export]
macro_rules! nested_validation_error {
	($field:expr, $code:expr, $errors:expr) => {
		::validy::core::ValidationError::Node(::validy::core::NestedValidationError {
			field: std::borrow::Cow::from($field),
			code: std::borrow::Cow::from($code),
			errors: $errors,
		})
	};
}
