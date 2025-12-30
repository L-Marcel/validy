use std::{
	borrow::Cow,
	sync::{Arc, LazyLock},
};

use moka::sync::Cache;
use regex::{Error, Regex};

use crate::settings::ValidationSettings;

pub struct RegexManager {
	cache: Cache<Cow<'static, str>, Arc<Regex>>,
}

static INSTANCE: LazyLock<RegexManager> = LazyLock::new(|| {
	let cache = ValidationSettings::get().regex_cache.clone();
	RegexManager { cache }
});

impl RegexManager {
	pub fn get_uncached(pattern: impl Into<Cow<'static, str>>) -> Result<Regex, Error> {
		Regex::new(&pattern.into())
	}

	pub fn get_or_create(pattern: impl Into<Cow<'static, str>>) -> Result<Arc<Regex>, Error> {
		let key = pattern.into();

		if let Some(regex) = INSTANCE.cache.get(&key) {
			return Ok(regex.clone());
		}

		let key_for_regex = key.clone();
		match INSTANCE
			.cache
			.entry(key)
			.or_try_insert_with(|| Regex::new(&key_for_regex).map(Arc::new))
		{
			Ok(entry) => Ok(entry.value().clone()),
			Err(arc_erro) => Err((*arc_erro).clone()),
		}
	}

	pub fn remove(pattern: impl Into<Cow<'static, str>>) {
		INSTANCE.cache.remove(&pattern.into());
	}
}
