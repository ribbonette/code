use ribbon_util::id_marker::{ GroupMarker, GroupRoleMarker };
use serde::{ Deserialize, Serialize };
use twilight_model::id::Id;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct CriteriaModel {
	pub items: Vec<CriteriaItem>
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CriteriaItem {
	GroupMembership {
		group_id: Id<GroupMarker>
	},
	GroupMembershipRole {
		group_id: Id<GroupMarker>,
		role_id: Id<GroupRoleMarker>
	},
	ValidAccount
}