#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Cache: {0}")]
	Cache(#[from] ribbon_cache::Error),

	#[error("Command: {0}")]
	Command(#[from] ribbon_commands_core::Error),

	#[error("Command Core: {0}")]
	CommandCore(#[from] ribbon_commands_core::CoreError),

	#[error("I/O: {0}")]
	Io(#[from] std::io::Error),

	#[error("Model: {0}")]
	Model(#[from] ribbon_models::Error),

	#[error("Reqwest: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("JSON: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Twilight HTTP Deserialise: {0}")]
	TwilightHttpDeserialise(#[from] twilight_http::response::DeserializeBodyError),

	#[error("Twilight HTTP: {0}")]
	TwilightHttp(#[from] twilight_http::Error),

	#[error("SQLx: {0}")]
	Sqlx(#[from] sqlx::Error),

	#[error("Syncing: {0}")]
	Syncing(#[from] ribbon_syncing::Error)
}

pub type Result<T> = core::result::Result<T, Error>;