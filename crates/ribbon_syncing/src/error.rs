#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Cache: {0}")]
	Cache(#[from] ribbon_cache::Error),

	#[error("Reqwest: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("Twilight HTTP: {0}")]
	TwilightHttp(#[from] twilight_http::Error)
}

pub type Result<T> = core::result::Result<T, Error>;