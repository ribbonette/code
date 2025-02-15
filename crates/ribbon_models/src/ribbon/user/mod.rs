use futures::TryStreamExt;
use ribbon_util::acquire_db_pool;
use serde::Serialize;
use twilight_model::id::{
	marker::UserMarker,
	Id
};

pub mod roblox_account;
pub use roblox_account::RobloxAccountModel;

pub mod session;
pub use session::SessionModel;

use crate::Result;

#[derive(Clone, Serialize)]
pub struct UserModel {
	pub id: Id<UserMarker>
}

impl UserModel {
	pub async fn get(user_id: Id<UserMarker>) -> Result<Option<Self>> {
		Self::get_many(&[user_id])
			.await
			.map(|x| x.into_iter().next())
	}

	pub async fn get_or_insert(user_id: Id<UserMarker>) -> Result<Self> {
		let db_pool = acquire_db_pool()?;
		Ok(match Self::get(user_id).await? {
			Some(x) => x,
			None => {
				sqlx::query!(
					"
					INSERT INTO users (id)
					VALUES ($1)
					",
					user_id.get() as i64
				)
					.execute(db_pool)
					.await?;
				Self {
					id: user_id
				}
			}
		})
	}

	pub async fn get_many(user_ids: &[Id<UserMarker>]) -> Result<Vec<Self>> {
		let db_pool = acquire_db_pool()?;
		let user_ids: Vec<i64> = user_ids
			.iter()
			.map(|x| x.get() as i64)
			.collect();
		Ok(sqlx::query!(
			"
			SELECT id
			FROM users
			WHERE id = ANY($1)
			",
			&user_ids
		)
			.fetch(db_pool)
			.try_fold(Vec::new(), |mut acc, record| {
				acc.push(Self {
					id: Id::new(record.id as u64)
				});

				async move { Ok(acc) }
			})
			.await?
		)
	}
}