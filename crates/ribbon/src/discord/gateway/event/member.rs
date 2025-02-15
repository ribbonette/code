use ribbon_cache::CACHE;
use twilight_model::gateway::payload::incoming::{ MemberAdd, MemberRemove, MemberUpdate };

use crate::Result;

pub async fn member_add(member_add: MemberAdd) -> Result<()> {
	let cache_key = (member_add.guild_id, member_add.user.id);
	CACHE
		.discord
		.members
		.insert(cache_key, member_add.member.into());

	Ok(())
}

pub async fn member_update(member_update: MemberUpdate) -> Result<()> {
	let cache_key = (member_update.guild_id, member_update.user.id);
	if let Some(mut member) = CACHE.discord.members.get_mut(&cache_key) {
		member.update(&member_update);
	}

	Ok(())
}

pub async fn member_remove(member_remove: MemberRemove) -> Result<()> {
	let cache_key = (member_remove.guild_id, member_remove.user.id);
	CACHE
		.discord
		.members
		.remove(&cache_key);

	Ok(())
}