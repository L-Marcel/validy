use syn::{
	Error, Ident, Result, Token, parenthesized,
	parse::{ParseBuffer, ParseStream},
};

pub fn remove_parens(input: ParseStream) -> Result<ParseBuffer> {
	let content: ParseBuffer<'_>;
	parenthesized!(content in input);
	Ok(content)
}

pub trait ArgParser: Default {
	const POSITIONAL_KEYS: &'static [&'static str];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()>;

	fn apply_positional(&mut self, index: usize, input: ParseStream) -> Result<()> {
		let name = Self::POSITIONAL_KEYS
			.get(index)
			.ok_or_else(|| Error::new(input.span(), "too many positional args"))?;

		self.apply_value(name, input)
	}
}

pub fn parse_attrs<T: ArgParser>(input: &ParseBuffer<'_>) -> Result<T> {
	let max_args = T::POSITIONAL_KEYS.len();
	let mut args = T::default();
	let mut args_count = 0;
	let mut index = 0;

	while !input.is_empty() {
		if args_count >= max_args {
			return Err(Error::new(input.span(), "too many args"));
		}

		if input.peek(Ident) && input.peek2(Token![=]) {
			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;
			args.apply_value(&key.to_string(), input)?;
		} else {
			args.apply_positional(index, input)?;
			index += 1;
		}

		args_count += 1;
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
		}
	}

	Ok(args)
}
