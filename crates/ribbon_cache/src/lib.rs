#![feature(let_chains)]
use once_cell::sync::Lazy;

use discord::DiscordCache;
use ribbon::RibbonCache;
use roblox::RobloxCache;

pub mod error;
pub mod discord;
pub mod ribbon;
pub mod roblox;

pub use error::{ Error, Result };

#[derive(Default)]
pub struct Cache {
	pub discord: DiscordCache,
	pub ribbon: RibbonCache,
	pub roblox: RobloxCache
}

pub static CACHE: Lazy<Cache> = Lazy::new(Cache::default);