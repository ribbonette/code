#![feature(const_async_blocks, type_alias_impl_trait)]
use once_cell::sync::{ Lazy, OnceCell };
use sqlx::PgPool;
use twilight_http::{ client::InteractionClient, Client };
use twilight_model::id::{ marker::ApplicationMarker, Id };

pub mod error;
pub use error::Error;
pub mod fetch;
pub use fetch::*;

pub mod id_marker;

// TODO: better environment variable handling
pub static DASHBOARD_URL: &str = env!("DASHBOARD_URL");
pub static FRONTEND_URL: &str = env!("FRONTEND_URL");
pub static WEBSITE_URL: &str = env!("WEBSITE_URL");

pub static DISCORD_CLIENT: Lazy<Client> = Lazy::new(|| Client::new(env!("DISCORD_BOT_TOKEN").to_owned()));
pub static DISCORD_INTERACTION_CLIENT: Lazy<InteractionClient> = Lazy::new(||
	DISCORD_CLIENT.interaction(*DISCORD_APP_ID)
);

pub static DISCORD_APP_ID: Lazy<Id<ApplicationMarker>> = Lazy::new(|| env!("DISCORD_APP_ID").to_owned().parse().unwrap());

pub static ROBLOX_APP_ID: Lazy<u64> = Lazy::new(|| env!("ROBLOX_APP_ID").to_owned().parse().unwrap());
pub static ROBLOX_APP_SECRET: &str = env!("ROBLOX_APP_SECRET");
pub static ROBLOX_OPEN_CLOUD_KEY: &str = env!("ROBLOX_OPEN_CLOUD_KEY");

pub static PG_POOL: OnceCell<PgPool> = OnceCell::new();

pub fn acquire_db_pool<'a>() -> Result<&'a PgPool, Error> {
	match PG_POOL.get() {
		Some(x) => Ok(x),
		None => Err(Error::DatabaseNotConnected)
	}
}