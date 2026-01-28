pub fn capitalize(value: &mut String) {
	use heck::ToTitleCase;
	*value = value.to_title_case();
}

pub fn camel_case(value: &mut String) {
	use heck::ToUpperCamelCase;
	*value = value.to_upper_camel_case();
}

pub fn lower_camel_case(value: &mut String) {
	use heck::ToLowerCamelCase;
	*value = value.to_lower_camel_case()
}

pub fn snake_case(value: &mut String) {
	use heck::ToSnakeCase;
	*value = value.to_snake_case()
}

pub fn shouty_snake_case(value: &mut String) {
	use heck::ToShoutySnakeCase;
	*value = value.to_shouty_snake_case()
}

pub fn kebab_case(value: &mut String) {
	use heck::ToKebabCase;
	*value = value.to_kebab_case()
}

pub fn shouty_kebab_case(value: &mut String) {
	use heck::ToShoutyKebabCase;
	*value = value.to_shouty_kebab_case()
}

pub fn train_case(value: &mut String) {
	use heck::ToTrainCase;
	*value = value.to_train_case();
}
