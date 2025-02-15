use log::{ info, warn };
use twilight_model::gateway::{
	payload::outgoing::update_presence::UpdatePresencePayload,
	presence::{ Status, Activity, ActivityType }
};
use twilight_gateway::{ Shard, Intents, ShardId, StreamExt, ConfigBuilder, MessageSender, EventTypeFlags };

pub mod event;

pub fn initialise() -> MessageSender {
	info!("initialising discord gateway");

	let config = ConfigBuilder::new(
		env!("DISCORD_BOT_TOKEN").to_string(),
			Intents::GUILDS |
			Intents::GUILD_MEMBERS
	)
		.presence(UpdatePresencePayload::new(vec![Activity {
			id: None,
			url: None,
			name: "burgers".into(),
			kind: ActivityType::Custom,
			emoji: None,
			flags: None,
			party: None,
			state: Some(std::env::var("DISCORD_STATUS_TEXT").unwrap_or("bringing roblox to discord!".into())),
			assets: None,
			buttons: vec![],
			details: None,
			secrets: None,
			instance: None,
			created_at: None,
			timestamps: None,
			application_id: None
		}], false, None, Status::Online).unwrap())
		.build();
	let mut shard = Shard::with_config(ShardId::ONE, config);
	let message_sender = shard.sender();
	tokio::spawn(async move {
		while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
			let Ok(event) = item else {
				warn!("error receiving event {}", item.unwrap_err());
				continue;
			};
	
			event::handle_event(event);
		}
	});

	message_sender
}