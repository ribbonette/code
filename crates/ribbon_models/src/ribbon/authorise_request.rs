use rand::{ distr::Alphanumeric, Rng };
use serde::Serialize;
use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};

use crate::discord::InteractionRef;

#[derive(Serialize)]
pub struct AuthoriseRequestModel {
	pub id: String,
	pub kind: AuthoriseRequestKind,
	
	#[serde(skip)]
	pub interaction: InteractionRef
}

impl AuthoriseRequestModel {
	fn new_id() -> String {
		rand::rng()
			.sample_iter(Alphanumeric)
			.take(24)
			.map(char::from)
			.collect()
	}
	
	pub fn new(interaction: impl Into<InteractionRef>, request_kind: AuthoriseRequestKind) -> Self {
		Self {
			id: Self::new_id(),
			kind: request_kind,
			
			interaction: interaction.into()
		}
	}
	
	pub fn add_roblox_account(interaction: impl Into<InteractionRef>) -> Self {
		Self::new(interaction, AuthoriseRequestKind::AddRobloxAccount)
	}
	
	pub fn add_roblox_communities(interaction: impl Into<InteractionRef>) -> Self {
		Self::new(interaction, AuthoriseRequestKind::AddRobloxCommunities)
	}
	
	pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
		self.interaction.guild_id
	}

	pub fn user_id(&self) -> Option<Id<UserMarker>> {
		self.interaction.user_id
	}
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoriseRequestKind {
	AddRobloxAccount,
	AddRobloxCommunities
}