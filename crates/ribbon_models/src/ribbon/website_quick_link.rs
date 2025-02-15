use rand::{ distr::Alphanumeric, Rng };
use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};

pub struct WebsiteQuickLinkModel {
	pub id: String,
	pub origin_server_id: Option<Id<GuildMarker>>,
	pub origin_user_id: Id<UserMarker>
}

impl WebsiteQuickLinkModel {
	fn new_id() -> String {
		rand::rng()
			.sample_iter(Alphanumeric)
			.take(24)
			.map(char::from)
			.collect()
	}
	
	pub fn new(user_id: Id<UserMarker>, server_id: Option<Id<GuildMarker>>) -> Self {
		Self {
			id: Self::new_id(),
			origin_server_id: server_id,
			origin_user_id: user_id
		}
	}
}