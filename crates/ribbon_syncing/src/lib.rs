#![feature(let_chains, type_alias_impl_trait)]
use dashmap::DashSet;
use ribbon_cache::CACHE;
use ribbon_emojis::Emoji;
use ribbon_models::{
	discord::InteractionRef,
	ribbon::{
		member_link::{
			connector::Connector,
			criteria::CriteriaItem
		},
		AuthoriseRequestModel
	}
};
use ribbon_util::{ DISCORD_CLIENT, DISCORD_INTERACTION_CLIENT, WEBSITE_URL };
use std::fmt::{ Display, Formatter };
use twilight_http::request::AuditLogReason;
use twilight_model::{
	channel::message::{
		component::{ ActionRow, Button, ButtonStyle },
		MessageFlags
	},
	http::interaction::{ InteractionResponse, InteractionResponseData, InteractionResponseType },
	id::{
		marker::{ GuildMarker, RoleMarker, UserMarker },
		Id
	}
};
use twilight_util::builder::InteractionResponseDataBuilder;

pub mod error;

pub use error::*;

pub struct SyncOperation {
	guild_id: Id<GuildMarker>,
	user_id: Id<UserMarker>,

	interaction: Option<(InteractionRef, bool)>
}

// TODO: queue system, of some sorts? i think that would be good
impl SyncOperation {
	pub fn from_interaction(interaction: impl Into<InteractionRef>, is_acknowledged: bool) -> Self {
		let interaction_ref: InteractionRef = interaction.into();
		Self {
			guild_id: interaction_ref.guild_id.unwrap(),
			user_id: interaction_ref.user_id.unwrap(),

			interaction: Some((interaction_ref, is_acknowledged))
		}
	}

	async fn execute(self) -> Result<SyncOperationResult> {
		let guild_id = self.guild_id;
		let user_id = self.user_id;

		let link_ids = CACHE
			.ribbon
			.user_roblox_accounts(user_id)
			.await?;
		let Some(link_id) = link_ids.into_iter().next() else {
			if let Some((interaction,_)) = self.interaction {
				let callback_url = {
					let request = AuthoriseRequestModel::add_roblox_account(interaction.clone());
					let request = CACHE
						.ribbon
						.authorise_requests
						.entry(request.id.clone())
						.insert(request);
					format!("{}/roblox_request/{}", WEBSITE_URL.as_str(), request.key())
				};
				DISCORD_INTERACTION_CLIENT
					.create_response(interaction.id, &interaction.token, &InteractionResponse {
						kind: InteractionResponseType::ChannelMessageWithSource,
						data: Some(InteractionResponseData {
							components: Some(vec![ActionRow {
								components: vec![Button {
									custom_id: None,
									disabled: false,
									emoji: Some(Emoji::IconRoblox.into()),
									label: Some("Connect Roblox Account".into()),
									sku_id: None,
									style: ButtonStyle::Link,
									url: Some(callback_url)
								}.into()]
							}.into()]),
							content: Some({
								let guild = CACHE
									.discord
									.guild(guild_id)
									.unwrap();
								format!(
									"
									## {}{}  Ribbon welcomes you to {}!\n\
									This server uses Ribbon to sync member profiles (roles, etc.) with the Roblox Platform.\n\
									Since you're new to Ribbon, you'll need to connect a Roblox Account to continue. Don't fret—it's quick, easy, and secure!
									",
									Emoji::HandWaving,
									Emoji::IconRibbon,
									guild.name
								)
							}),
							flags: Some(MessageFlags::EPHEMERAL),
							..Default::default()
						})
					})
					.await?;
				return Ok(SyncOperationResult::Cancelled);
			} else {
				unimplemented!()
			}
		};

		let roblox_id = CACHE
			.ribbon
			.roblox_account(link_id)
			.await?
			.roblox_id;
		
		if let Some((interaction, acknowledged)) = &self.interaction && !acknowledged {
			DISCORD_INTERACTION_CLIENT
				.create_response(interaction.id, &interaction.token, &InteractionResponse {
					kind: InteractionResponseType::DeferredChannelMessageWithSource,
					data: Some(InteractionResponseDataBuilder::new()
						.flags(MessageFlags::EPHEMERAL)
						.build()
					)
				})
				.await?;
		}
			
		let member_link_ids = CACHE
			.ribbon
			.server_member_links(guild_id)
			.await?;

		let mut role_changes: Vec<RoleChange> = Vec::new();
		for member_link_id in member_link_ids {
			if let Some(member_link) = CACHE.ribbon.member_link(member_link_id) {
				let criteria_items = member_link.criteria.items.clone();
				drop(member_link);

				let mut is_criteria_met = true;
				for item in criteria_items {
					let is_item_met = match item {
						CriteriaItem::GroupMembership { group_id } => {
							let group_ids = CACHE
								.roblox
								.user_memberships(roblox_id)
								.await?;

							group_ids
								.iter()
								.any(|x| x == &group_id)
						},
						CriteriaItem::GroupMembershipRole { group_id, role_id } => {
							CACHE
								.roblox
								.user_memberships(roblox_id)
								.await?;
							
							CACHE
								.roblox
								.membership(group_id, roblox_id)
								.is_some_and(|x| x.role_id() == role_id)
						},
						CriteriaItem::ValidAccount => true
					};
					if !is_item_met {
						is_criteria_met = false;
						break;
					}
				}

				if let Some(member_link) = CACHE.ribbon.member_link(member_link_id) {
					for connector in &member_link.connectors.items {
						match connector {
							Connector::Roles { target_role_ids } => RoleChange::extend_with_many(
								&mut role_changes,
								if is_criteria_met { RoleChangeKind::Assign } else { RoleChangeKind::Remove },
								target_role_ids
							),
							_ => unimplemented!()
						}
					}
				}
			}
		}

		let roles_changed = if !role_changes.is_empty() {
			let new_roles = {
				let member = CACHE
					.discord
					.member(guild_id, user_id)
					.await?;
				RoleChange::apply_changes(&mut role_changes, &member.roles)
			};
			if let Some(new_roles) = new_roles {
				DISCORD_CLIENT
					.update_guild_member(guild_id, user_id)
					.reason("sync operation")
					.roles(&new_roles)
					.await?;
				true
			} else { false }
		} else { false };

		let profile_changed = roles_changed;
		let operation_result = SyncOperationResult::Success {
			profile_changed,
			role_changes
		};

		if let Some((interaction, _acknowledged)) = self.interaction {
			DISCORD_INTERACTION_CLIENT
				.update_response(&interaction.token)
				.content(Some(&operation_result.to_string()))
				.components(Some(&[ActionRow {
					components: vec![
						Button {
							custom_id: Some("sync_again".into()),
							disabled: false,
							emoji: Some(Emoji::ArrowClockwise.into()),
							label: Some("Sync Again".into()),
							sku_id: None,
							style: ButtonStyle::Primary,
							url: None
						}.into(),
						Button {
							custom_id: None,
							disabled: false,
							emoji: Some(Emoji::IconDiscord.into()),
							label: Some("Get Support".into()),
							sku_id: None,
							style: ButtonStyle::Link,
							url: Some("https://discord.com/invite/rs3r4dQu9P".into())
						}.into()
					]
				}.into()]))
				.await?;
		}

		Ok(operation_result)
	}
}

pub type SyncOperationFuture = impl Future<Output = Result<SyncOperationResult>>;

impl IntoFuture for SyncOperation {
	type IntoFuture = SyncOperationFuture;
	type Output = Result<SyncOperationResult>;

	fn into_future(self) -> Self::IntoFuture {
		self.execute()
	}
}

#[derive(Debug)]
pub struct RoleChange {
	pub kind: RoleChangeKind,
	pub role_id: Id<RoleMarker>
}

impl RoleChange {
	pub fn new(kind: RoleChangeKind, role_id: Id<RoleMarker>) -> Self {
		Self { kind, role_id }
	}

	pub fn extend_with_many(target: &mut Vec<RoleChange>, kind: RoleChangeKind, role_ids: &[Id<RoleMarker>]) {
		target.extend(
			role_ids
				.iter()
				.map(|role_id| Self::new(kind.clone(), *role_id))	
		);
	}

	pub fn is_assign(&self) -> bool {
		matches!(self.kind, RoleChangeKind::Assign)
	}

	pub fn is_remove(&self) -> bool {
		matches!(self.kind, RoleChangeKind::Remove)
	}

	pub fn apply_changes(role_changes: &mut Vec<RoleChange>, roles: &[Id<RoleMarker>]) -> Option<Vec<Id<RoleMarker>>> {
		let new_roles: DashSet<_> = roles
			.iter()
			.copied()
			.collect();
		role_changes
			.retain(|role_change| match role_change.kind {
				RoleChangeKind::Assign => new_roles
					.insert(role_change.role_id),
				RoleChangeKind::Remove => new_roles
					.remove(&role_change.role_id)
					.is_some()
			});
		
		if role_changes.is_empty() { None } else {
			Some(new_roles
				.into_iter()
				.collect()
			)
		}
	}
}

impl Display for RoleChange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let role = CACHE
			.discord
			.role(self.role_id);
		let role_name = role
			.as_ref()
			.map_or("", |x| &x.name);
		write!(f, "{} {role_name}", match self.kind {
			RoleChangeKind::Assign => "+",
			RoleChangeKind::Remove => "-"
		})
	}
}

#[derive(Clone, Debug)]
pub enum RoleChangeKind {
	Assign,
	Remove
}

pub enum SyncOperationResult {
	Success {
		profile_changed: bool,
		role_changes: Vec<RoleChange>
	},
	Cancelled
}

impl Display for SyncOperationResult {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Success { profile_changed, role_changes } => if *profile_changed {
				writeln!(f, "## {}  Server profile updated!", Emoji::PersonFillCheck)?;
				if role_changes.is_empty() {
					write!(f, "uhhhh")
				} else {
					writeln!(f, "Your roleset has been updated, see changes below.\n```diff")?;
					for role_change in role_changes {
						writeln!(f, "{role_change}")?;
					}
					write!(f, "```")
				}
			} else {
				write!(f,
					"
					## {}  Server profile already up to par!\n\
					Your profile seems up-to-date, nothing needs changing at this time!
					",
					Emoji::PersonFillExclamation
				)
			},
			Self::Cancelled => write!(f, "cancelled")
		}
	}
}