use actix_web::HttpResponse;
use ribbon_cache::CACHE;
use twilight_model::id::{ marker::GuildMarker, Id };

use crate::{
	error::ErrorModelKind,
	routes::v1::roblox_callback::RobloxUser,
	Result
};

pub async fn finished_guild(guild_id: Id<GuildMarker>, roblox_user: &RobloxUser) -> Result<HttpResponse> {
	let roblox_name = roblox_user
		.name
		.as_ref()
		.or(
			roblox_user
				.preferred_username
				.as_ref()
		);
	let mut body = include_str!("finished_guild.html")
		.to_string();
	if let Some(avatar_url) = &roblox_user.picture {
		body = body.replace("{{ roblox_avatar }}", avatar_url);
	}
	if let Some(roblox_name) = roblox_name {
		body = body.replace("{{ roblox_name }}", roblox_name);
	}

	let guild = CACHE
		.discord
		.guild(guild_id)
		.ok_or_else(|| ErrorModelKind::Cache.model())?;
	if let Some(avatar_url) = guild.avatar_url() {
		body = body.replace("{{ guild_avatar }}", &avatar_url);
	}
	body = body.replace("{{ guild_name }}", &guild.name);
	
	Ok(HttpResponse::Ok()
		.append_header(("content-type", "text/html; charset=utf-8"))
		.body(body)
	)
}