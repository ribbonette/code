#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Database connection has not been established")]
	DatabaseNotConnected
}

pub type Result<T> = core::result::Result<T, Error>;