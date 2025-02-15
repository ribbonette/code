use serde::{ Deserialize, Serialize };
use twilight_model::id::{
	marker::RoleMarker,
	Id
};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ConnectorsModel {
	pub items: Vec<Connector>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Connector {
	Nickname,
	Roles {
		target_role_ids: Vec<Id<RoleMarker>>
	}
}