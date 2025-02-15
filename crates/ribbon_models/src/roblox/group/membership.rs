use ribbon_util::{ id_marker::GroupMarker, ROBLOX_OPEN_CLOUD_KEY, get_json };
use serde::Deserialize;
use twilight_model::id::Id;

use crate::Result;

#[derive(Deserialize)]
pub struct GroupMemberships {
	#[serde(rename = "groupMemberships")]
	pub items: Vec<MembershipModel>,
	#[serde(default, rename = "nextPageToken")]
	pub next_page_token: Option<String>
}

#[derive(Deserialize)]
pub struct MembershipModel {
	path: String,
	role: String
}

impl MembershipModel {
	pub async fn get_user_many(user_id: u64) -> Result<Vec<Self>> {
		let response: GroupMemberships = get_json("https://apis.roblox.com/cloud/v2/groups/-/memberships")
			.header("x-api-key", ROBLOX_OPEN_CLOUD_KEY)
			.query(&[
				("filter", format!("user == 'users/{user_id}'")),
				("maxPageSize", "100".to_string())
			])
			.await?;
		Ok(response.items)
	}

	// TODO: not this
	pub fn id(&self) -> &str {
		self
			.path
			.split('/')
			.nth(3)
			.unwrap()
	}

	// TODO: not this
	pub fn group_id(&self) -> Id<GroupMarker> {
		self
			.role
			.split('/')
			.nth(1)
			.unwrap()
			.parse()
			.unwrap()
	}

	// TODO: not this
	pub fn role_id(&self) -> u64 {
		self
			.role
			.split('/')
			.nth(3)
			.unwrap()
			.parse()
			.unwrap()
	}
}