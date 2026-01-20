use std::{
	borrow::Cow,
	sync::{Arc, OnceLock},
};

#[cfg(feature = "pattern")]
use moka::sync::Cache;
#[cfg(feature = "pattern")]
use regex::Regex;

pub struct ValidationSettings {
	#[cfg(feature = "pattern")]
	pub regex_cache: Cache<Cow<'static, str>, Arc<Regex>>,
}

impl Default for ValidationSettings {
	fn default() -> Self {
		Self {
			#[cfg(feature = "pattern")]
			regex_cache: Cache::<Cow<'static, str>, Arc<Regex>>::builder()
				.max_capacity(100)
				.initial_capacity(10)
				.build(),
		}
	}
}

static SETTINGS: OnceLock<ValidationSettings> = OnceLock::new();

impl ValidationSettings {
	pub fn get() -> &'static ValidationSettings {
		SETTINGS.get_or_init(ValidationSettings::default)
	}

	pub fn init(settings: ValidationSettings) -> Result<(), ValidationSettings> {
		SETTINGS.set(settings)
	}
}
