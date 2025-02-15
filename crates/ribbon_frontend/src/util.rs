use actix_cors::Cors;
use serde::{ Deserialize, Deserializer };
use serde_aux::field_attributes::StringOrVecToVec;
use std::str::FromStr;

pub fn default_cors() -> Cors {
	Cors::default()
		.allow_any_origin()
		.allow_any_header()
		.allow_any_method()
		.supports_credentials()
		.max_age(3600)
}

pub fn deserialise_vec_from_spaced_string_or_vec<'de, D: Deserializer<'de>, T: FromStr + Deserialize<'de> + 'static>(deserializer: D) -> core::result::Result<Vec<T>, D::Error>
where
	<T as FromStr>::Err: std::fmt::Display
{
	StringOrVecToVec::with_separator(|c| c == ' ').into_deserializer()(deserializer)
}