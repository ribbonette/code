use actix_web::HttpRequest;
use base64::{ prelude::BASE64_STANDARD, Engine };
use p384::ecdsa::{ signature::Verifier, Signature, VerifyingKey };
use ribbon_util::acquire_db_pool;
use twilight_model::id::{
	marker::UserMarker,
	Id
};

use crate::{ Error, Result };

pub struct SessionModel {
	pub id: u64,
	pub public_key: VerifyingKey,
	pub user_id: Id<UserMarker>
}

impl SessionModel {
	pub async fn get(session_id: u64) -> Result<Option<Self>> {
		let db_pool = acquire_db_pool()?;
		Ok(if let Some(record) = sqlx::query!(
			"
			SELECT public_key, user_id
			FROM user_sessions
			WHERE id = $1
			",
			session_id as i64
		)
			.fetch_optional(db_pool)
			.await?
		{
			Some(Self {
				id: session_id,
				public_key: VerifyingKey::from_sec1_bytes(&record.public_key)?,
				user_id: Id::new(record.user_id as u64)
			})
		} else { None })
	}

	pub async fn insert(user_id: Id<UserMarker>, public_key: VerifyingKey) -> Result<Self> {
		let db_pool = acquire_db_pool()?;
		let record = sqlx::query!(
			"
			INSERT INTO user_sessions (public_key, user_id)
			VALUES ($1, $2)
			RETURNING id
			",
			&*public_key.to_sec1_bytes(),
			user_id.get() as i64
		)
			.fetch_one(db_pool)
			.await?;
		Ok(Self {
			id: record.id as u64,
			public_key,
			user_id
		})
	}

	pub fn verify_request(&self, request: &HttpRequest, body: &[u8]) -> Result<()> {
		let headers = request.headers();
		let raw_signature = headers.get("haku-sig")
			.ok_or(Error::MissingSignature)?;

		let decoded_signature = BASE64_STANDARD.decode(raw_signature)?;
		let signature = Signature::from_slice(&decoded_signature)?;

		let data = format!("{} {};{}", request.method(), request.path(), BASE64_STANDARD.encode(body));
		self.public_key.verify(data.as_bytes(), &signature)?;

		Ok(())
	}
}