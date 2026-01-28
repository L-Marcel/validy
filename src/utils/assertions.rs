use ::validy::core::ValidationErrors;
use pretty_assertions::assert_eq;
use std::fmt::Debug;

#[macro_export]
macro_rules! assert_errors {
	($result:expr, $object:expr, $( $errors:tt )*) => {
	  let expected = ::validy::validation_errors! $( $errors )*;
		::validy::utils::assertions::assert_validation_errors(
			&$result,
			&$object,
			&expected
		);
	};
}

#[macro_export]
macro_rules! assert_parsed {
	($result:expr, $object:expr, $expected:expr) => {
		::validy::utils::assertions::assert_parsed_validation(&$result, &$object, &$expected);
	};
}

#[macro_export]
macro_rules! assert_modification {
	($result:expr, $object:expr, $expected:expr) => {
		::validy::utils::assertions::assert_modification(&$result, &$object, &$expected);
	};
}

#[macro_export]
macro_rules! assert_validation {
	($result:expr, $object:expr) => {
		::validy::utils::assertions::assert_validation(&$result, &$object);
	};
}

pub fn assert_validation_errors<T: Debug, O: Debug>(
	result: &Result<T, ValidationErrors>,
	object: &O,
	expected: &ValidationErrors,
) {
	match result {
		Ok(value) => panic!(
			"Expected Err({:#?}), received Ok({:#?}) from {:#?} validation.",
			expected, value, object
		),
		Err(value) => assert_eq!(value, expected, "Result did not match expectations for {:#?}.", object),
	}
}

pub fn assert_parsed_validation<T: Debug + PartialEq, O: Debug>(
	result: &Result<T, ValidationErrors>,
	object: &O,
	expected: &T,
) {
	match result {
		Ok(value) => assert_eq!(value, expected, "Result did not match expectations for {:#?}.", object),
		Err(value) => panic!(
			"Expected Ok({:#?}), received Err({:#?}) from {:#?} vlidation.",
			expected, value, object
		),
	}
}

pub fn assert_validation<T: Debug>(result: &Result<(), ValidationErrors>, object: &T) {
	if let Err(error) = result {
		panic!(
			"Expected Ok(()), received Err({:#?}) from {:#?} validation.",
			error, object
		);
	}
}

pub fn assert_modification<T: Debug + PartialEq, O: Debug>(result: &T, expected: &T, object: &O) {
	assert_eq!(result, expected, "Result did not match expectations for {:#?}.", object)
}
