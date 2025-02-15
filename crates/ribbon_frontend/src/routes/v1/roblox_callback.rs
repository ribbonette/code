use actix_web::{
	web,
	HttpResponse, Responder,
	get
};
use chrono::{ TimeDelta, Utc };
use ribbon_cache::CACHE;
use ribbon_models::ribbon::{
	open_cloud_authorisation::{ OpenCloudAuthorisationMetadata, OpenCloudAuthorisationModel },
	user::RobloxAccountModel
};
use ribbon_syncing::SyncOperation;
use ribbon_util::{
	acquire_db_pool,
	get_json, post_json
};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::collections::HashMap;

use crate::{
	error::ErrorModelKind,
	util::deserialise_vec_from_spaced_string_or_vec,
	Result
};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(roblox_callback);
}

#[derive(Deserialize)]
struct CallbackQuery {
	code: Option<String>,
	state: Option<String>
}

#[derive(Deserialize)]
struct BasicToken {
	access_token: String,
	refresh_token: String,
	expires_in: u32,
	token_type: String,
	#[serde(deserialize_with = "deserialise_vec_from_spaced_string_or_vec", rename = "scope")]
	scopes: Vec<String>
}

#[derive(Deserialize)]
pub struct RobloxUser {
	#[serde(deserialize_with = "deserialize_number_from_string")]
	pub sub: u64,
	pub name: Option<String>,
	pub preferred_username: Option<String>,
	pub profile: Option<String>,
	pub picture: Option<String>
}

const ROBLOX_APP_ID: &str = env!("ROBLOX_APP_ID");
const ROBLOX_APP_SECRET: &str = env!("ROBLOX_APP_SECRET");

#[get("roblox_callback")]
async fn roblox_callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
	let code = query.code
		.clone()
		.ok_or(ErrorModelKind::InvalidQuery)?;

	let params = HashMap::from([
		("client_id", ROBLOX_APP_ID.into()),
		("client_secret", ROBLOX_APP_SECRET.into()),
		("code", code),
		("grant_type", "authorization_code".into())
	]);

	let tokens: BasicToken = post_json("https://apis.roblox.com/oauth/v1/token")
		.form(&params)
		.await?;

	let roblox_user: RobloxUser = get_json("https://apis.roblox.com/oauth/v1/userinfo")
		.header("authorization", format!("{} {}", tokens.token_type, tokens.access_token))
		.await?;

	if let Some(state) = &query.state {
		if
			let Some(token) = state.strip_prefix("r_") &&
			let Some((_,request)) = CACHE.ribbon.authorise_requests.remove(token)
		{
			let guild_id = request.guild_id().unwrap();
			let user_id = request.user_id().unwrap();
			CACHE
				.ribbon
				.user(user_id)
				.await?;

			let db_pool = acquire_db_pool()?;

			let expires_at = Utc::now()
				.checked_add_signed(TimeDelta::seconds(tokens.expires_in as i64))
				.unwrap();
			let roblox_id = roblox_user.sub;
			let metadata = OpenCloudAuthorisationMetadata {
				scopes: tokens.scopes
			};

			let new_record = sqlx::query!(
				"
				INSERT INTO open_cloud_authorisations (author_id, access_token, refresh_token, token_type, metadata, expires_at)
				VALUES ($1, $2, $3, $4, $5, $6)
				RETURNING id
				",
				user_id.get() as i64,
				&tokens.access_token,
				&tokens.refresh_token,
				&tokens.token_type,
				serde_json::to_value(&metadata).unwrap(),
				&expires_at
			)
				.fetch_one(db_pool)
				.await?;

			let authorisation_id = new_record.id as u64;
			CACHE
				.ribbon
				.roblox_authorisations.insert(authorisation_id, OpenCloudAuthorisationModel {
					id: authorisation_id,
					author_id: user_id,

					access_token: tokens.access_token,
					refresh_token: tokens.refresh_token,
					token_type: tokens.token_type,
					metadata,

					expires_at
				});

			let new_record = sqlx::query!(
				"
				INSERT INTO user_roblox_accounts (authorisation_id, roblox_id, user_id)
				VALUES ($1, $2, $3)
				RETURNING id
				",
				new_record.id,
				roblox_id as i64,
				user_id.get() as i64
			)
				.fetch_one(db_pool)
				.await?;

			CACHE
				.ribbon
				.roblox_accounts.insert(roblox_id, RobloxAccountModel {
					id: new_record.id as u64,
					roblox_id
				});
			CACHE
				.ribbon
				.user_roblox_accounts
				.entry(user_id)
				.or_default()
				.insert(roblox_id);

			SyncOperation::from_interaction(request.interaction, true)
				.await?;

			return crate::templates::connection_callback::finished_guild(guild_id, &roblox_user)
				.await;
		}
	}

	Ok(HttpResponse::Ok().into())
}