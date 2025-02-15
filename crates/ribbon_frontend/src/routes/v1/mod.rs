use actix_web::web;

pub mod auth;
pub mod internal;
pub mod roblox_callback;
pub mod servers;
pub mod users;

pub fn config(config: &mut web::ServiceConfig) {
	config.service(
		web::scope("v1")
			.wrap(crate::util::default_cors())
			.configure(auth::config)
			.configure(internal::config)
			.configure(roblox_callback::config)
			.configure(servers::config)
			.configure(users::config)
	);
}