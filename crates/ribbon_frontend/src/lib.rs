#![feature(duration_constructors, let_chains)]
use actix_web::{
	error::InternalError,
	middleware::Logger,
	web,
	App, HttpResponse, HttpServer
};
use log::info;
use once_cell::sync::Lazy;
use ribbon_cache::CACHE;

mod auth;
pub mod error;
pub mod routes;
mod templates;
mod util;

pub type Result<T> = core::result::Result<T, error::ErrorModel>;

pub async fn setup_frontend() -> std::io::Result<()> {
	let bind_addr = std::env::var("BIND_ADDRESS")
		.expect("BIND_ADDRESS is not defined");
	info!("starting ribbon_frontend on {bind_addr}");
	
	Lazy::force(&CACHE);

	HttpServer::new(|| {
        App::new()
			.wrap(Logger::new("%r  →  %s, %b bytes, took %Dms"))
            .configure(routes::v1::config)
			.app_data(web::JsonConfig::default().error_handler(|error,_| InternalError::from_response(
				"",
				HttpResponse::BadRequest()
					.content_type("application/json")
					.body(format!(r#"{{"error":"json error: {error}"}}"#)),
			).into()))
			.default_service(web::get().wrap(util::default_cors()).to(routes::default::default))
    })
		.bind(bind_addr)?
		.run()
		.await
}