use futures::TryStreamExt;
use ribbon_util::acquire_db_pool;
use twilight_model::id::{
	marker::UserMarker,
	Id
};

use crate::Result;

pub struct RobloxAccountModel {
	pub id: u64,
	pub roblox_id: u64
}

impl RobloxAccountModel {
	pub async fn get_user_many(user_id: Id<UserMarker>) -> Result<Vec<Self>> {
		let db_pool = acquire_db_pool()?;
		Ok(sqlx::query!(
			"
			SELECT id, roblox_id
			FROM user_roblox_accounts
			WHERE user_id = $1
			",
			user_id.get() as i64
		)
			.fetch(db_pool)
			.try_fold(Vec::new(), |mut acc, record| {
				acc.push(Self {
					id: record.id as u64,
					roblox_id: record.roblox_id as u64
				});

				async move { Ok(acc) }
			})
			.await?
		)
	}
}