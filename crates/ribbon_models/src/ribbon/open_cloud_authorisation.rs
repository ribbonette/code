use chrono::{ DateTime, Utc };
use ribbon_util::acquire_db_pool;
use serde::{ Deserialize, Serialize };
use twilight_model::id::{
	marker::UserMarker,
	Id
};

use crate::Result;

pub struct OpenCloudAuthorisationModel {
	pub id: u64,
	pub author_id: Id<UserMarker>,

	pub access_token: String,
	pub refresh_token: String,
	pub token_type: String,
	pub metadata: OpenCloudAuthorisationMetadata,

	pub expires_at: DateTime<Utc>
}

impl OpenCloudAuthorisationModel {
	pub async fn get(authorisation_id: u64) -> Result<Option<Self>> {
		let db_pool = acquire_db_pool()?;
		Ok(sqlx::query!(
			"
			SELECT author_id, access_token, refresh_token, token_type, metadata, expires_at
			FROM open_cloud_authorisations
			WHERE id = $1
			",
			authorisation_id as i64
		)
			.fetch_optional(db_pool)
			.await?
			.map(|record| Self {
				id: authorisation_id,
				author_id: Id::new(record.author_id as u64),

				access_token: record.access_token,
				refresh_token: record.refresh_token,
				token_type: record.token_type,
				metadata: serde_json::from_value(record.metadata).unwrap(),

				expires_at: record.expires_at
			})
		)
	}
}

#[derive(Deserialize, Serialize)]
pub struct OpenCloudAuthorisationMetadata {
	pub scopes: Vec<String>
}