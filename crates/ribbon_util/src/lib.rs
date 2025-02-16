#![feature(const_async_blocks, type_alias_impl_trait)]
use once_cell::sync::{ Lazy, OnceCell };
use sqlx::PgPool;
use std::env::var;
use twilight_http::{ client::InteractionClient, Client };
use twilight_model::id::{ marker::ApplicationMarker, Id };

pub mod error;
pub use error::Error;
pub mod fetch;
pub use fetch::*;

pub mod id_marker;

pub static DASHBOARD_URL: Lazy<String> = Lazy::new(|| var("DASHBOARD_URL").unwrap());
pub static FRONTEND_URL: Lazy<String> = Lazy::new(|| var("FRONTEND_URL").unwrap());
pub static WEBSITE_URL: Lazy<String> = Lazy::new(|| var("WEBSITE_URL").unwrap());

pub static DISCORD_CLIENT: Lazy<Client> = Lazy::new(|| Client::new(var("DISCORD_BOT_TOKEN").unwrap()));
pub static DISCORD_INTERACTION_CLIENT: Lazy<InteractionClient> = Lazy::new(||
	DISCORD_CLIENT.interaction(*DISCORD_APP_ID)
);

pub static DISCORD_APP_ID: Lazy<Id<ApplicationMarker>> = Lazy::new(|| var("DISCORD_APP_ID").unwrap().parse().unwrap());

pub static ROBLOX_APP_ID: Lazy<u64> = Lazy::new(|| var("ROBLOX_APP_ID").unwrap().parse().unwrap());
pub static ROBLOX_APP_SECRET: Lazy<String> = Lazy::new(|| var("ROBLOX_APP_SECRET").unwrap());
pub static ROBLOX_OPEN_CLOUD_KEY: Lazy<String> = Lazy::new(|| var("ROBLOX_OPEN_CLOUD_KEY").unwrap());

pub static PG_POOL: OnceCell<PgPool> = OnceCell::new();

pub fn acquire_db_pool<'a>() -> Result<&'a PgPool, Error> {
	match PG_POOL.get() {
		Some(x) => Ok(x),
		None => Err(Error::DatabaseNotConnected)
	}
}