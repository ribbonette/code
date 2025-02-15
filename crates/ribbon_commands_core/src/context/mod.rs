use ribbon_models::discord::InteractionRef;
use ribbon_util::DISCORD_INTERACTION_CLIENT;
use twilight_http::request::application::interaction::UpdateResponse;
use twilight_model::{
	application::interaction::application_command::CommandDataOption,
	http::interaction::InteractionResponse,
	id::{ marker::{ ChannelMarker, GuildMarker, UserMarker }, Id }
};

use crate::{ Interaction, Result };

pub mod defer_builder;
pub use defer_builder::DeferBuilder;

pub mod reply_builder;
pub use reply_builder::ReplyBuilder;

pub struct Context {
	pub interaction: Interaction,
	pub options: Vec<CommandDataOption>
}

impl Context {
	pub fn new(interaction: Interaction) -> Self {
		let options = interaction.options().into_iter().cloned().collect();
		Self {
			interaction,
			options
		}
	}

	pub fn author_id(&self) -> Option<Id<UserMarker>> {
		self.interaction.user_id
	}

	pub fn channel_id(&self) -> Option<Id<ChannelMarker>> {
		self.interaction.channel.as_ref().map(|x| x.id)
	}

	pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
		self.interaction.guild_id
	}

	pub fn defer(&self) -> DeferBuilder {
		DeferBuilder::new(self)
	}

	pub fn update(&self) -> UpdateResponse<'_> {
		DISCORD_INTERACTION_CLIENT.update_response(&self.interaction.token)
	}

	pub fn reply(&self, content: impl Into<String>) -> ReplyBuilder {
		ReplyBuilder::new(self, content)
	}

	pub async fn response(&self, response: InteractionResponse) -> Result<()> {
		DISCORD_INTERACTION_CLIENT
			.create_response(
				self.interaction.id,
				&self.interaction.token,
				&response
			)
			.await?;

		Ok(())
	}
}

impl From<&Context> for InteractionRef {
	fn from(value: &Context) -> Self {
		Self {
			id: value.interaction.id,
			kind: value.interaction.kind,
			token: value.interaction.token.clone(),
			guild_id: value.guild_id(),
			user_id: value.author_id()
		}
	}
}