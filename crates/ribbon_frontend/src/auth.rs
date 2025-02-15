use actix_web::{
	cookie::{ time::Duration, Cookie },
	HttpRequest
};
use dashmap::mapref::one::Ref;
use jsonwebtoken::{ Algorithm, DecodingKey, EncodingKey, Validation };
use once_cell::sync::Lazy;
use ribbon_cache::CACHE;
use ribbon_models::ribbon::user::SessionModel;
use serde::Deserialize;
use std::ops::Deref;

use crate::{ error::ErrorModelKind, Result };

pub const AUTH_JWT_DOMAIN: &str = env!("AUTH_JWT_DOMAIN");
pub const AUTH_JWT_DURATION: Duration = Duration::days(365);
pub const AUTH_JWT_KEY: &[u8] = env!("AUTH_JWT_KEY").as_bytes();
pub static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(AUTH_JWT_KEY));
pub static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_secret(AUTH_JWT_KEY));
pub static VALIDATION: Lazy<Validation> = Lazy::new(|| {
	let mut validation = Validation::new(Algorithm::HS256);
	validation.required_spec_claims.clear();
	validation.validate_exp = true;
	validation
});

pub struct SessionOption<'a> {
	inner: Option<Ref<'a, u64, SessionModel>>
}

impl<'a> SessionOption<'a> {
	pub fn required(self) -> Result<Ref<'a, u64, SessionModel>> {
		self
			.inner
			.ok_or(ErrorModelKind::MissingCredentials.model())
	}
}

impl<'a> Deref for SessionOption<'a> {
	type Target = Option<Ref<'a, u64, SessionModel>>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<'a> From<Option<Ref<'a, u64, SessionModel>>> for SessionOption<'a> {
	fn from(value: Option<Ref<'a, u64, SessionModel>>) -> Self {
		Self { inner: value }
	}
}

pub async fn get_session_from_request(request: &HttpRequest) -> Result<SessionOption<'_>> {
	Ok(if let Some(jwt_token_cookie) = get_authorisation_header(request) {
		let jwt_token = jwt_token_cookie.value();
		let session_id = match CACHE.ribbon.jwt_sessions.get(jwt_token).as_deref().copied() {
			Some(session_id) => session_id,
			None => get_session_id_from_jwt_token(jwt_token)
				.await?
		};
		
		Some(CACHE
			.ribbon
			.session(session_id)
			.await?
		)
	} else { None }.into())
}

#[derive(Deserialize)]
struct Claims {
	session_id_temp: u64
}

async fn get_session_id_from_jwt_token(jwt_token: &str) -> Result<u64> {
	let token = jsonwebtoken::decode::<Claims>(jwt_token, &DECODING_KEY, &VALIDATION)
		.map_err(|x| {
			println!("{x}");
			ErrorModelKind::InvalidCredentials.model()
		})?;
	Ok(token.claims.session_id_temp)
}

fn get_authorisation_header(request: &HttpRequest) -> Option<Cookie<'static>> {
	request
		.cookie("ribbon-user-session")
}