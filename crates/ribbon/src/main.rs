#![feature(const_async_blocks, is_none_or, let_chains, try_blocks, type_alias_impl_trait)]
use clap::Parser;
use env_logger::Env;
use log::info;
use once_cell::sync::Lazy;
use sqlx::PgPool;
use std::pin::Pin;
use ribbon_cache::CACHE;
use ribbon_commands::commands::COMMANDS;
use ribbon_util::{ DISCORD_APP_ID, DISCORD_CLIENT, DISCORD_INTERACTION_CLIENT, PG_POOL };
use twilight_gateway::CloseFrame;
use twilight_model::{
	application::command::CommandType,
	oauth::ApplicationFlags
};

use discord::command::ApplicationCommand;
pub use error::Result;

mod discord;
mod error;

#[derive(Parser)]
struct Args {
	#[clap(long, short)]
    update_commands: bool
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

	info!("starting Ribbon v{}", env!("CARGO_PKG_VERSION"));

	rustls::crypto::aws_lc_rs::default_provider()
		.install_default()
		.unwrap();
	
	let args = Args::parse();
	if args.update_commands {
		let mut commands: Vec<ApplicationCommand> = vec![];
		for command in COMMANDS.iter() {
			if command.is_slash {
				commands.push(ApplicationCommand::new(command, CommandType::ChatInput)?);
			}
			if command.is_message {
				commands.push(ApplicationCommand::new(command, CommandType::Message)?);
			}
			if command.is_user {
				commands.push(ApplicationCommand::new(command, CommandType::User)?);
			}
		}

		DISCORD_CLIENT
			.request::<()>(
				twilight_http::request::Request::builder(&twilight_http::routing::Route::SetGlobalCommands {
					application_id: DISCORD_APP_ID.get()
				})
					.json(&commands)
					.build()?
			)
			.await?;

		info!("successfully updated global commands");
	} else {
		info!("establishing database connection pool...");
		let pg_pool = match PgPool::connect(env!("DATABASE_URL")).await {
			Ok(x) => x,
			Err(error) => panic!("failed to establish a connection to the database, is it offline?\n{error}")
		};
		PG_POOL
			.set(pg_pool)
			.unwrap();

		Lazy::force(&CACHE);
		Lazy::force(&COMMANDS);
		Lazy::force(&DISCORD_INTERACTION_CLIENT); // also evaluates DISCORD_CLIENT & DISCORD_APP_ID
		Pin::static_ref(&discord::DISCORD_APP_COMMANDS).await;

		let application = DISCORD_CLIENT
			.current_user_application()
			.await?
			.model()
			.await?;
		if application.flags.is_none_or(|x| !x.contains(ApplicationFlags::GATEWAY_GUILD_MEMBERS_LIMITED)) {
			info!("Server Members Intent not enabled, updating application...");
			DISCORD_CLIENT
				.update_current_user_application()
				.flags(ApplicationFlags::GATEWAY_GUILD_MEMBERS_LIMITED)
				.await?;
		}

		let message_sender = discord::gateway::initialise();
		ribbon_frontend::setup_frontend()
			.await?;

		info!("shutdown signal received, saving stuff...");
		
		message_sender.close(CloseFrame::NORMAL).unwrap();

		info!("now shutting down...");
	}

	Ok(())
}