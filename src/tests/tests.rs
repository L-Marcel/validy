use crate::{
	core::{ValidationError, ValidationErrors},
	functions::email::validate_email,
};

struct Data {
	pub email: String,
}

fn trim<'a, T>(value: &'a str) -> T
where
	T: From<&'a str>,
{
	value.trim().into()
}

//#field_id = self.#field_name
//#field_id = temp_#field_name

impl Data {
	fn validate(&mut self) -> Result<(), ValidationErrors> {
		let mut errors = Vec::<ValidationError>::new();

		//modify trim
		let mut a = trim::<String>(&self.email);

		//validate email
		if let Err(e) = validate_email(&self.email, "email", "email", "invalid email format") {
			errors.push(e);
		}

		if errors.is_empty() {
			Ok(())
		} else {
			let map: ValidationErrors = errors
				.into_iter()
				.map(|e| match e {
					ValidationError::Node(e) => (e.field.clone(), ValidationError::Node(e)),
					ValidationError::Leaf(e) => (e.field.clone(), ValidationError::Leaf(e)),
				})
				.collect();

			Err(map)
		}
	}
}
