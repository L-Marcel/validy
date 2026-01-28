use std::borrow::Cow;

use ::validy::{
	core::{ValidationError, ValidationErrors},
	settings::FailureMode,
};

pub fn can_continue(errors: &ValidationErrors, mode: FailureMode, field_name: &str) -> bool {
	!matches!(mode, FailureMode::FailOncePerField) || !errors.contains_key(field_name)
}

pub fn should_fail_fast(errors: &ValidationErrors, mode: FailureMode, field_name: &str) -> bool {
	matches!(mode, FailureMode::FailFast) && errors.contains_key(field_name)
}

pub fn append_error(
	errors: &mut ValidationErrors,
	error: ValidationError,
	mode: FailureMode,
	field_name: impl Into<Cow<'static, str>>,
) {
	let entry = errors.entry(field_name.into()).or_default();

	if matches!(mode, FailureMode::LastFailPerField) {
		entry.clear();
	}

	entry.push(error);
}
