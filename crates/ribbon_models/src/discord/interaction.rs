use twilight_model::{
	application::interaction::{ Interaction, InteractionType },
	id::{
		marker::{ GuildMarker, InteractionMarker, UserMarker },
		Id
	}
};

#[derive(Clone)]
pub struct InteractionRef {
	pub id: Id<InteractionMarker>,
	pub kind: InteractionType,
	pub token: String,
	pub guild_id: Option<Id<GuildMarker>>,
	pub user_id: Option<Id<UserMarker>>
}

impl From<Interaction> for InteractionRef {
	fn from(value: Interaction) -> Self {
		Self {
			id: value.id,
			kind: value.kind,
			token: value.token.clone(),
			guild_id: value.guild_id,
			user_id: value.author_id()
		}
	}
}