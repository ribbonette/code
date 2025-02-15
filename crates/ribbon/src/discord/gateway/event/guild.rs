use ribbon_cache::CACHE;
use ribbon_util::DISCORD_CLIENT;
use twilight_model::{
	gateway::payload::incoming::{ GuildCreate, GuildUpdate, GuildDelete },
	guild::Role,
	id::{
		marker::GuildMarker,
		Id
	}
};

use crate::Result;

fn add_roles_to_cache(guild_id: Id<GuildMarker>, roles: &[Role]) {
	for role in roles {
		let role_id = role.id;
		if !CACHE.discord.roles.contains_key(&role_id) {
			CACHE.discord.roles.insert(role_id, role.clone().into());
		}
	}

	let role_ids = roles
		.iter()
		.map(|x| x.id);
	CACHE
		.discord
		.guild_roles
		.entry(guild_id)
		.or_default()
		.extend(role_ids);
}

pub fn guild_create(guild_create: GuildCreate) -> Result<()> {
	match guild_create {
		GuildCreate::Available(guild) => {
			let guild_id = guild.id;
			add_roles_to_cache(guild_id, &guild.roles);

			CACHE.discord.guilds.insert(guild_id, guild.into());
		},
		GuildCreate::Unavailable(guild) => if ! guild.unavailable {
			tokio::spawn(async move {
				let guild_id = guild.id;
				let guild = DISCORD_CLIENT
					.guild(guild_id)
					.await
					.unwrap()
					.model()
					.await
					.unwrap();
				add_roles_to_cache(guild_id, &guild.roles);

				CACHE.discord.guilds.insert(guild_id, guild.into());
			});
		}
	}
	
	Ok(())
}

pub fn guild_update(guild_update: GuildUpdate) -> Result<()> {
	let guild_id = guild_update.id;
	if let Some(mut guild) = CACHE.discord.guilds.get_mut(&guild_id) {
		guild.update(&guild_update);
	}

	add_roles_to_cache(guild_id, &guild_update.roles);

	Ok(())
}

pub fn guild_delete(guild_delete: GuildDelete) -> Result<()> {
	let guild_id = guild_delete.id;
	CACHE.discord.guilds.remove(&guild_id);

	if let Some((_,role_ids)) = CACHE.discord.guild_roles.remove(&guild_id) {
		for role_id in role_ids {
			CACHE.discord.roles.remove(&role_id);
		}
	}

	Ok(())
}