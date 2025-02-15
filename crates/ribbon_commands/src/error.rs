#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Cache: {0}")]
	Cache(#[from] ribbon_cache::Error),

	#[error("SQLx: {0}")]
	Sqlx(#[from] sqlx::Error),

	#[error("Syncing: {0}")]
	Syncing(#[from] ribbon_syncing::Error),

	#[error("Unknown")]
	Unknown
}

pub type Result<T> = core::result::Result<T, Error>;