#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Base64: {0}")]
	Base64(#[from] base64::DecodeError),
	
	#[error("ECDSA: {0}")]
	Ecdsa(#[from] p384::ecdsa::Error),

	#[error("Missing signature")]
	MissingSignature,

	#[error("Reqwest: {0}")]
	Reqwest(#[from] reqwest::Error),
	
	#[error("Serde JSON: {0}")]
	SerdeJson(#[from] serde_json::Error),

	#[error("SQLx: {0}")]
	Sqlx(#[from] sqlx::Error),

	#[error("Twilight Http: {0}")]
	TwilightHttp(#[from] twilight_http::Error),

	#[error("Twilight Http Deserialise: {0}")]
	TwilightHttpDeserialise(#[from] twilight_http::response::DeserializeBodyError),

	#[error("Util: {0}")]
	Util(#[from] ribbon_util::Error)
}

pub type Result<T> = core::result::Result<T, Error>;