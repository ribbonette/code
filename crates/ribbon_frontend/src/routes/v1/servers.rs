use actix_web::{
	web,
	HttpRequest,
	HttpResponse,
	get, patch, post
};
use ribbon_cache::CACHE;
use ribbon_models::ribbon::member_link::{ ConnectorsModel, CriteriaModel, MemberLinkModel };
use ribbon_util::acquire_db_pool;
use serde::Deserialize;
use twilight_model::{
	guild::Permissions,
	id::{
		marker::{ GuildMarker, UserMarker },
		Id
	}
};

use crate::{
	auth::get_session_from_request,
	error::{ ErrorModelKind, ResourceKind },
	Result
};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(web::scope("server")
		.service(server_get)
		.service(web::scope("{server_id}")
			.service(server_member_links)
			.service(server_member_link_create)
			.service(web::scope("member_link")
				.service(server_member_link_update)
			)
		)
	);
}

async fn check_user_permissions(server_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<()> {
	if let Some(guild) = CACHE.discord.guild(server_id) && guild.owner_id == user_id {
		return Ok(());
	}

	let member = CACHE
		.discord
		.member(server_id, user_id)
		.await?;
	for role_id in &member.roles {
		if let Some(role) = CACHE.discord.role(*role_id) {
			if role.permissions.contains(Permissions::MANAGE_GUILD) || role.permissions.contains(Permissions::ADMINISTRATOR) {
				return Ok(());
			}
		}
	}

	Err(ErrorModelKind::MissingPermission.model())
}

#[get("{server_id}")]
async fn server_get(request: HttpRequest, path: web::Path<u64>) -> Result<HttpResponse> {
	let server_id: Id<GuildMarker> = Id::new_checked(*path)
		.ok_or(ErrorModelKind::InvalidParams)?;
	
	let user_id = get_session_from_request(&request)
		.await?
		.required()?
		.user_id;
	check_user_permissions(server_id, user_id)
		.await?;
	
	let server = CACHE
		.ribbon
		.server(server_id)
		.await?;
	Ok(HttpResponse::Ok().json(server.value()))
}

#[get("member_links")]
async fn server_member_links(request: HttpRequest, path: web::Path<u64>) -> Result<HttpResponse> {
	let server_id: Id<GuildMarker> = Id::new_checked(*path)
		.ok_or(ErrorModelKind::InvalidParams)?;
	let server = CACHE
		.ribbon
		.server(server_id)
		.await?;

	let user_id = get_session_from_request(&request)
		.await?
		.required()?
		.user_id;
	check_user_permissions(server.id, user_id)
		.await?;

	let member_link_ids = CACHE
		.ribbon
		.server_member_links(server.id)
		.await?;
	let member_links: Vec<_> = CACHE
		.ribbon
		.member_links(&member_link_ids)
		.into_iter()
		.map(|x| serde_json::to_value(&*x).unwrap())
		.collect();
	Ok(HttpResponse::Ok().json(member_links))
}

#[derive(Deserialize)]
struct CreateMemberLink {
	#[serde(default)]
	connectors: ConnectorsModel,
	#[serde(default)]
	criteria: CriteriaModel,
	display_name: String
}

#[post("member_links")]
async fn server_member_link_create(request: HttpRequest, path: web::Path<u64>, payload: web::Json<CreateMemberLink>) -> Result<HttpResponse> {
	let server_id: Id<GuildMarker> = Id::new_checked(*path)
		.ok_or(ErrorModelKind::InvalidParams)?;

	let db_pool = acquire_db_pool()?;
	let user_id = get_session_from_request(&request)
		.await?
		.required()?
		.user_id;
	check_user_permissions(server_id, user_id)
		.await?;

	let payload = payload.into_inner();
	let new_record = sqlx::query!(
		"
		INSERT INTO server_member_links (connectors, criteria, display_name, server_id)
		VALUES ($1, $2, $3, $4)
		RETURNING id
		",
		serde_json::to_value(&payload.connectors)?,
		serde_json::to_value(&payload.criteria)?,
		payload.display_name,
		server_id.get() as i64
	)
		.fetch_one(db_pool)
		.await?;

	let new_member_link_id = new_record.id as u64;
	let new_member_link = MemberLinkModel {
		connectors: payload.connectors,
		criteria: payload.criteria,
		display_name: payload.display_name.clone(),
		id: new_member_link_id
	};

	Ok(if let Some(server_member_link_ids) = CACHE.ribbon.server_member_links.get_mut(&server_id) {
		server_member_link_ids.insert(new_member_link_id);

		let new_member_link_ref = CACHE
			.ribbon
			.member_links
			.entry(new_member_link_id)
			.insert(new_member_link);

		HttpResponse::Ok()
			.json(&*new_member_link_ref)
	} else {
		HttpResponse::Ok()
			.json(new_member_link)
	})
}

#[derive(Deserialize)]
struct UpdateMemberLink {
	connectors: Option<ConnectorsModel>,
	criteria: Option<CriteriaModel>,
	display_name: Option<String>
}

#[patch("{member_link_id}")]
async fn server_member_link_update(request: HttpRequest, path: web::Path<(u64, u64)>, payload: web::Json<UpdateMemberLink>) -> Result<HttpResponse> {
	let server_id: Id<GuildMarker> = Id::new_checked(path.0)
		.ok_or(ErrorModelKind::InvalidParams)?;

	let db_pool = acquire_db_pool()?;
	let user_id = get_session_from_request(&request)
		.await?
		.required()?
		.user_id;
	check_user_permissions(server_id, user_id)
		.await?;

	let member_link_id = path.1;

	let server_member_link_ids = CACHE
		.ribbon
		.server_member_links(server_id)
		.await?;
	if !server_member_link_ids.contains(&member_link_id) {
		return Err(ErrorModelKind::not_found(ResourceKind::ServerMemberLink, Some(member_link_id)));
	}

	let mut transaction = db_pool
		.begin()
		.await?;
	if let Some(connectors) = &payload.connectors {
		if let Some(mut member_link) = CACHE.ribbon.member_links.get_mut(&member_link_id) {
			member_link.connectors = connectors.clone();
		}

		sqlx::query!(
			"
			UPDATE server_member_links
			SET connectors = $2
			WHERE id = $1
			",
			member_link_id as i64,
			serde_json::to_value(connectors)?
		)
			.execute(&mut *transaction)
			.await?;
	}

	if let Some(criteria) = &payload.criteria {
		if let Some(mut member_link) = CACHE.ribbon.member_links.get_mut(&member_link_id) {
			member_link.criteria = criteria.clone();
		}

		sqlx::query!(
			"
			UPDATE server_member_links
			SET criteria = $2
			WHERE id = $1
			",
			member_link_id as i64,
			serde_json::to_value(criteria)?
		)
			.execute(&mut *transaction)
			.await?;
	}

	if let Some(display_name) = &payload.display_name {
		if let Some(mut member_link) = CACHE.ribbon.member_links.get_mut(&member_link_id) {
			member_link.display_name = display_name.clone();
		}

		sqlx::query!(
			"
			UPDATE server_member_links
			SET display_name = $2
			WHERE id = $1
			",
			member_link_id as i64,
			display_name
		)
			.execute(&mut *transaction)
			.await?;
	}

	transaction
		.commit()
		.await?;

	Ok(HttpResponse::Ok().finish())
}