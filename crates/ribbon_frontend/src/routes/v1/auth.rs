use actix_multipart::form::{ bytes::Bytes, text::Text, MultipartForm };
use actix_web::{
	cookie::{ time::OffsetDateTime, Cookie, SameSite },
	web,
	HttpResponse,
	post
};
use p384::ecdsa::VerifyingKey;
use ribbon_cache::CACHE;
use ribbon_models::ribbon::user::{ SessionModel, UserModel };
use serde::Serialize;
use twilight_model::id::{
	marker::{ GuildMarker, UserMarker },
	Id
};

use crate::{
	auth::{ AUTH_JWT_DOMAIN, AUTH_JWT_DURATION, ENCODING_KEY },
	error::{ ErrorModelKind, ResourceKind },
	Result
};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(web::scope("auth")
		.service(quick_link)
	);
}

#[derive(Serialize)]
struct JwtClaims {
	session_id_temp: u64,
	sub: Id<UserMarker>
}

#[derive(MultipartForm)]
struct QuickLinkPayload {
	#[multipart(limit = "97 B")]
	public_key: Bytes,
	#[multipart(limit = "24 B")]
	token: Text<String>
}

#[derive(Serialize)]
struct QuickLinkResponse {
	quick_link_server_id: Option<Id<GuildMarker>>,
	user: UserModel
}

#[post("quick_link")]
async fn quick_link(MultipartForm(payload): MultipartForm<QuickLinkPayload>) -> Result<HttpResponse> {
	let token = payload.token.0;
	let Some((_,quick_link)) = CACHE.ribbon.website_quick_links.remove(&token) else {
		return Err(ErrorModelKind::not_found(ResourceKind::WebsiteQuickLink, Some(token)));
	};
	let server_id = quick_link.origin_server_id;
	let user_id = quick_link.origin_user_id;

	let public_key = VerifyingKey::from_sec1_bytes(&payload.public_key.data)?;
	let new_session = SessionModel::insert(user_id, public_key)
		.await?;
	
	let new_jwt_claims = JwtClaims {
		session_id_temp: new_session.id,
		sub: user_id
	};
	let new_jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &new_jwt_claims, &ENCODING_KEY)?;
	let mut new_jwt_cookie_builder = Cookie::build("ribbon-user-session", new_jwt)
		.expires(OffsetDateTime::now_utc().checked_add(AUTH_JWT_DURATION).unwrap())
		.http_only(false)
		.path("/")
		.same_site(SameSite::None);
	if AUTH_JWT_DOMAIN.is_empty() {
		new_jwt_cookie_builder = new_jwt_cookie_builder.secure(false);
	} else {
		new_jwt_cookie_builder = new_jwt_cookie_builder.domain(AUTH_JWT_DOMAIN.clone());
	}
	
	let new_jwt_cookie = new_jwt_cookie_builder.finish();

	let user = CACHE
		.ribbon
		.user(user_id)
		.await?;

	let response = QuickLinkResponse {
		quick_link_server_id: server_id,
		user: user.clone()
	};
	Ok(HttpResponse::Ok()
		.cookie(new_jwt_cookie)
		.json(response)
	)
}