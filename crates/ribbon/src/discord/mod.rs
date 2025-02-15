use async_once_cell::Lazy;
use ribbon_util::DISCORD_INTERACTION_CLIENT;
use twilight_model::application::command::Command;

pub mod command;
pub mod gateway;
pub mod interactions;

pub type CommandsFuture = impl Future<Output = Vec<Command>> + Send;
pub static DISCORD_APP_COMMANDS: Lazy<Vec<Command>, CommandsFuture> = Lazy::new(async {
	DISCORD_INTERACTION_CLIENT.global_commands().await.unwrap().model().await.unwrap()
});