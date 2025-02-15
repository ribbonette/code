use dashmap::{
	mapref::one::Ref,
	DashMap, DashSet
};
use ribbon_models::discord::{
	guild::{ MemberModel, RoleModel },
	ChannelModel, EmojiModel, GuildModel
};
use twilight_model::id::{
	marker::{ ChannelMarker, EmojiMarker, GuildMarker, RoleMarker, UserMarker },
	Id
};	

use crate::Result;

#[derive(Default)]
pub struct DiscordCache {
	pub application_emojis: DashSet<Id<EmojiMarker>>,
	pub channels: DashMap<Id<ChannelMarker>, ChannelModel>,
	pub emojis: DashMap<Id<EmojiMarker>, EmojiModel>,
	pub emojis_mapped: DashMap<String, Id<EmojiMarker>>,
	pub guilds: DashMap<Id<GuildMarker>, GuildModel>,
	pub guild_roles: DashMap<Id<GuildMarker>, DashSet<Id<RoleMarker>>>,
	pub members: DashMap<(Id<GuildMarker>, Id<UserMarker>), MemberModel>,
	pub private_channels: DashMap<Id<UserMarker>, Id<ChannelMarker>>,
	pub roles: DashMap<Id<RoleMarker>, RoleModel>
}

impl DiscordCache {
	pub async fn channel(&self, channel_id: Id<ChannelMarker>) -> Result<Ref<'_, Id<ChannelMarker>, ChannelModel>> {
		Ok(match self.channels.get(&channel_id) {
			Some(model) => model,
			None => {
				let new_model = ChannelModel::get(channel_id)
					.await?;
				self.channels
					.entry(channel_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub fn emoji(&self, emoji_id: Id<EmojiMarker>) -> Option<Ref<'_, Id<EmojiMarker>, EmojiModel>> {
		self.emojis.get(&emoji_id)
	}

	pub fn emoji_mapped(&self, emoji_name: &str) -> Option<Id<EmojiMarker>> {
		self
			.emojis_mapped
			.get(emoji_name)
			.as_deref()
			.copied()
	}

	pub fn guild(&self, guild_id: Id<GuildMarker>) -> Option<Ref<'_, Id<GuildMarker>, GuildModel>> {
		self.guilds.get(&guild_id)
	}

	pub fn guild_roles(&self, guild_id: Id<GuildMarker>) -> Vec<Id<RoleMarker>> {
		if let Some(role_ids) = self.guild_roles.get(&guild_id) {
			role_ids
				.iter()
				.map(|x| *x)
				.collect()
		} else {
			Vec::new()
		}
	}

	pub async fn member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<Ref<'_, (Id<GuildMarker>, Id<UserMarker>), MemberModel>> {
		let key = (guild_id, user_id);
		Ok(match self.members.get(&key) {
			Some(model) => model,
			None => {
				let new_model = MemberModel::get(guild_id, user_id)
					.await?;
				self.members
					.entry(key)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn private_channel(&self, user_id: Id<UserMarker>) -> Result<Id<ChannelMarker>> {
		Ok(*match self.private_channels.get(&user_id) {
			Some(model) => model,
			None => {
				let new_model = ChannelModel::get_private(user_id)
					.await?;
				let new_model_id = new_model.id;

				self.channels.insert(new_model_id, new_model);
				self.private_channels.entry(user_id)
					.insert(new_model_id)
					.downgrade()
			}
		})
	}

	pub fn role(&self, role_id: Id<RoleMarker>) -> Option<Ref<'_, Id<RoleMarker>, RoleModel>> {
		self.roles.get(&role_id)
	}
}