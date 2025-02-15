use actix_web::{
	web,
	HttpResponse,
	get
};
use ribbon_cache::CACHE;

use crate::{
	error::{ ErrorModelKind, ResourceKind },
	Result
};

pub fn config(config: &mut web::ServiceConfig) {
	config.service(web::scope("internal")
		.service(web::scope("authorise_request")
			.service(authorise_request_get)
		)
	);
}

#[get("{request_id}")]
async fn authorise_request_get(path: web::Path<String>) -> Result<HttpResponse> {
	let request_id = path.into_inner();
	let Some(authorise_request) = CACHE
		.ribbon
		.authorise_request(&request_id) else {
			return Err(ErrorModelKind::not_found(ResourceKind::AuthoriseRequest, Some(request_id)));	
		};
	Ok(HttpResponse::Ok().json(authorise_request.value()))
}