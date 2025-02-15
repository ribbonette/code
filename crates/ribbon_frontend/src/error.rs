use std::fmt::Display;
use serde::Serialize;
use actix_web::{ http::StatusCode, HttpResponse };

#[derive(Debug, Serialize)]
pub struct ErrorModel {
	pub error: ErrorModelKind
}

impl Display for ErrorModel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("ErrorModel")
	}
}

impl std::error::Error for ErrorModel {}

impl actix_web::ResponseError for ErrorModel {
	fn status_code(&self) -> StatusCode {
		match self.error {
			ErrorModelKind::Cache |
			ErrorModelKind::Database |
			ErrorModelKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
			ErrorModelKind::NotFound { .. } => StatusCode::NOT_FOUND,
			ErrorModelKind::InvalidCredentials |
			ErrorModelKind::MissingCredentials => StatusCode::UNAUTHORIZED,
			ErrorModelKind::InvalidSignature |
			ErrorModelKind::InvalidParams |
			ErrorModelKind::InvalidQuery |
			ErrorModelKind::MissingSignature => StatusCode::BAD_REQUEST,
			ErrorModelKind::MissingPermission => StatusCode::FORBIDDEN
		}
	}

	fn error_response(&self) -> HttpResponse {
		HttpResponse::build(self.status_code()).json(self)
	}
}

impl From<ErrorModelKind> for ErrorModel {
	fn from(value: ErrorModelKind) -> Self {
		Self {
			error: value
		}
	}
}

impl From<ribbon_cache::Error> for ErrorModel {
	fn from(_value: ribbon_cache::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<ribbon_models::Error> for ErrorModel {
	fn from(_value: ribbon_models::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<ribbon_syncing::Error> for ErrorModel {
	fn from(_value: ribbon_syncing::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<jsonwebtoken::errors::Error> for ErrorModel {
	fn from(_value: jsonwebtoken::errors::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<p384::ecdsa::Error> for ErrorModel {
	fn from(_value: p384::ecdsa::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<reqwest::Error> for ErrorModel {
	fn from(_value: reqwest::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<ribbon_util::Error> for ErrorModel {
	fn from(_value: ribbon_util::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<serde_json::Error> for ErrorModel {
	fn from(_value: serde_json::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

impl From<sqlx::Error> for ErrorModel {
	fn from(_value: sqlx::Error) -> Self {
		println!("{_value}");
		ErrorModelKind::InternalError.model()
	}
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ErrorModelKind {
	Cache,
	Database,
	InternalError,
	InvalidParams,
	InvalidQuery,
	NotFound {
		resource_kind: ResourceKind,
		resource_reference: Option<String>
	},
	InvalidCredentials,
	MissingCredentials,
	InvalidSignature,
	MissingSignature,
	MissingPermission
}

impl ErrorModelKind {
	pub fn model(self) -> ErrorModel {
		self.into()
	}

	pub fn not_found(resource_kind: ResourceKind, resource_reference: Option<impl ToString>) -> ErrorModel {
		Self::NotFound {
			resource_kind,
			resource_reference: resource_reference.map(|x| x.to_string())
		}.into()
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
	AuthoriseRequest,
	Route,
	Server,
	ServerMemberLink,
	User,
	UserConnection,
	WebsiteQuickLink
}