use ::validy::settings::ValidationSettings;
use regex::{Error, Regex};
use std::{borrow::Cow, sync::Arc};

pub struct RegexManager {}

impl RegexManager {
	pub fn get_uncached(pattern: impl Into<Cow<'static, str>>) -> Result<Regex, Error> {
		Regex::new(&pattern.into())
	}

	pub fn get_or_create(pattern: impl Into<Cow<'static, str>>) -> Result<Arc<Regex>, Error> {
		let key = pattern.into();
		let cache = ValidationSettings::get_regex_cache();

		if let Some(regex) = cache.get(&key) {
			return Ok(regex.clone());
		}

		match cache
			.entry_by_ref(&key)
			.or_try_insert_with(|| Regex::new(&key).map(Arc::new))
		{
			Ok(entry) => Ok(entry.value().clone()),
			Err(arc_erro) => Err((*arc_erro).clone()),
		}
	}

	pub fn remove(pattern: impl Into<Cow<'static, str>>) {
		ValidationSettings::get_regex_cache().remove(&pattern.into());
	}
}
