use actix_web::{
	web,
	HttpResponse,
	post
};
use twilight_model::id::{
	marker::UserMarker,
	Id
};

use crate::{
	error::ErrorModelKind,
	Result
};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(web::scope("user")
		.service(user_get)
	);
}

#[post("{user_id}")]
async fn user_get(path: web::Path<u64>) -> Result<HttpResponse> {
	let _user_id: Id<UserMarker> = Id::new_checked(*path)
		.ok_or(ErrorModelKind::InvalidParams)?;
	// not implemented
	Ok(HttpResponse::Ok().finish())
}