use dashmap::{
	mapref::{
		multiple::RefMulti,
		one::{ Ref, RefMut }
	},
	DashMap, DashSet
};
use ribbon_models::ribbon::{
	user::{ RobloxAccountModel, SessionModel },
	AuthoriseRequestModel, MemberLinkModel, OpenCloudAuthorisationModel, ServerModel, UserModel, WebsiteQuickLinkModel
};
use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};	

use crate::{ Error, Result };

#[derive(Default)]
pub struct RibbonCache {
	pub authorise_requests: DashMap<String, AuthoriseRequestModel>,
	pub jwt_sessions: DashMap<String, u64>,
	pub member_links: DashMap<u64, MemberLinkModel>,
	pub roblox_accounts: DashMap<u64, RobloxAccountModel>,
	pub roblox_authorisations: DashMap<u64, OpenCloudAuthorisationModel>,
	pub servers: DashMap<Id<GuildMarker>, ServerModel>,
	pub server_member_links: DashMap<Id<GuildMarker>, DashSet<u64>>,
	pub sessions: DashMap<u64, SessionModel>,
	users: DashMap<Id<UserMarker>, UserModel>,
	pub user_roblox_accounts: DashMap<Id<UserMarker>, DashSet<u64>>,
	pub website_quick_links: DashMap<String, WebsiteQuickLinkModel>
}

impl RibbonCache {
	pub fn authorise_request(&self, token: &str) -> Option<Ref<'_, String, AuthoriseRequestModel>> {
		self.authorise_requests.get(token)
	}

	pub fn member_link(&self, member_link_id: u64) -> Option<Ref<'_, u64, MemberLinkModel>> {
		self.member_links.get(&member_link_id)
	}

	pub fn member_links(&self, member_link_ids: &[u64]) -> Vec<RefMulti<u64, MemberLinkModel>> {
		self.member_links
			.iter()
			.filter(|x| member_link_ids.contains(&x.id))
			.collect()
	}

	pub async fn roblox_account(&self, link_id: u64) -> Result<Ref<'_, u64, RobloxAccountModel>> {
		Ok(match self.roblox_accounts.get(&link_id) {
			Some(model) => model,
			None => unimplemented!()
		})
	}

	pub async fn roblox_authorisation(&self, authorisation_id: u64) -> Result<Ref<'_, u64, OpenCloudAuthorisationModel>> {
		Ok(match self.roblox_authorisations.get(&authorisation_id) {
			Some(model) => model,
			None => unimplemented!()
		})
	}

	pub async fn server(&self, guild_id: Id<GuildMarker>) -> Result<Ref<'_, Id<GuildMarker>, ServerModel>> {
		Ok(match self.servers.get(&guild_id) {
			Some(model) => model,
			None => self
				._insert_server(guild_id)
				.await?
				.downgrade()
		})
	}

	pub async fn server_mut(&self, guild_id: Id<GuildMarker>) -> Result<RefMut<Id<GuildMarker>, ServerModel>> {
		Ok(match self.servers.get_mut(&guild_id) {
			Some(model) => model,
			None => self
				._insert_server(guild_id)
				.await?
		})
	}

	pub async fn session(&self, session_id: u64) -> Result<Ref<'_, u64, SessionModel>> {
		Ok(match self.sessions.get(&session_id) {
			Some(model) => model,
			None => {
				let new_model = SessionModel::get(session_id)
					.await?
					.ok_or(Error::NotFound)?;
				self.
					sessions
					.entry(session_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn server_member_links(&self, guild_id: Id<GuildMarker>) -> Result<Vec<u64>> {
		Ok(match self.server_member_links.get(&guild_id) {
			Some(model) => model
				.iter()
				.map(|x| *x)
				.collect(),
			None => {
				let models = MemberLinkModel::get_server_many(guild_id)
					.await?;
				let model_ids: Vec<_> = models
					.iter()
					.map(|x| x.id)
					.collect();
				for model in models {
					self.member_links.insert(model.id, model);
				}

				self.server_member_links
					.entry(guild_id)
					.or_default()
					.extend(model_ids.clone());
				model_ids
			}
		})
	}

	async fn _insert_server(&self, guild_id: Id<GuildMarker>) -> Result<RefMut<Id<GuildMarker>, ServerModel>> {
		let new_model = ServerModel::get_or_insert(guild_id)
			.await?;
		Ok(self.servers
			.entry(guild_id)
			.insert(new_model)
		)
	}

	pub async fn user(&self, user_id: Id<UserMarker>) -> Result<Ref<'_, Id<UserMarker>, UserModel>> {
		Ok(match self.users.get(&user_id) {
			Some(model) => model,
			None => {
				let new_model = UserModel::get_or_insert(user_id)
					.await?;
				self
					.users
					.entry(user_id)
					.insert(new_model)
					.downgrade()
			}
		})
	}

	pub async fn user_roblox_accounts(&self, user_id: Id<UserMarker>) -> Result<Vec<u64>> {
		Ok(match self.user_roblox_accounts.get(&user_id) {
			Some(model) => model
				.iter()
				.map(|x| *x)
				.collect(),
			None => {
				let models = RobloxAccountModel::get_user_many(user_id)
					.await?;
				let model_ids: Vec<_> = models
					.iter()
					.map(|x| x.id)
					.collect();
				for model in models {
					self.roblox_accounts.insert(model.id, model);
				}

				self.user_roblox_accounts
					.entry(user_id)
					.or_default()
					.extend(model_ids.clone());
				model_ids
			}
		})
	}
	
	pub fn website_quick_link(&self, quick_link_token: &str) -> Option<Ref<'_, String, WebsiteQuickLinkModel>> {
		self.website_quick_links.get(quick_link_token)
	}
}