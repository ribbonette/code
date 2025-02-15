use ribbon_util::acquire_db_pool;
use serde::Serialize;
use twilight_model::id::{
	marker::GuildMarker,
	Id
};

use crate::Result;

pub mod integration;
pub use integration::IntegrationModel;

#[derive(Serialize)]
pub struct ServerModel {
	pub id: Id<GuildMarker>,
	pub display_name: String
}

impl ServerModel {
	pub async fn get(guild_id: Id<GuildMarker>) -> Result<Option<Self>> {
		let db_pool = acquire_db_pool()?;
		Ok(sqlx::query!(
			"
			SELECT id
			FROM servers
			WHERE id = $1
			",
			guild_id.get() as i64
		)
			.fetch_optional(db_pool)
			.await?
			.map(|record| Self {
				id: Id::new(record.id as u64),
				display_name: "placeholder".into()
			})
		)
	}
	
	pub async fn get_or_insert(guild_id: Id<GuildMarker>) -> Result<Self> {
		Ok(match Self::get(guild_id).await? {
			Some(x) => x,
			None => {
				let db_pool = acquire_db_pool()?;
				sqlx::query!(
					"
					INSERT INTO servers (id)
					VALUES ($1)
					",
					guild_id.get() as i64
				)
					.execute(db_pool)
					.await?;
				Self {
					id: guild_id,
					display_name: "placeholder".into()
				}
			}
		})
	}
}

impl From<Id<GuildMarker>> for ServerModel {
	fn from(value: Id<GuildMarker>) -> Self {
		Self {
			id: value,
			display_name: "placeholder".into()
		}
	}
}