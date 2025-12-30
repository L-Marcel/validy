use syn::{
	Result, parenthesized,
	parse::{ParseBuffer, ParseStream},
};

pub fn remove_parens(input: ParseStream) -> Result<ParseBuffer> {
	let content: ParseBuffer<'_>;
	parenthesized!(content in input);
	Ok(content)
}
