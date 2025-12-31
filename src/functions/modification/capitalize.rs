pub fn capitalize(value: &str) -> String {
	use heck::ToTitleCase;
	value.to_title_case()
}
