use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};

pub struct IntegrationModel {
	pub author_id: Id<UserMarker>,
	pub server_id: Id<GuildMarker>
}

pub enum IntegrationKind {
	
}