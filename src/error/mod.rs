use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type ServerResult<T> = core::result::Result<T, ServerError>;

#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerError {
	LoginFail,


	// Auth errors
	AuthFailNoAuthTokenCookie,
	AuthFailTokenWrongFormat,
	AuthFailCtxNotInRequestExt,

	// Model errors
	TicketDeleteFaiIdNotFound {id: u64}
}

impl IntoResponse for ServerError {
	fn into_response(self) -> axum::response::Response {
		println!("{:<12} - {self:?}", "INTO_RES");
		
		// Create a placeholder Axum Response
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the error into the resonse
		response.extensions_mut().insert(self);


		response
	}
}

impl ServerError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			// -- Auth
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			},

			// -- Model
			Self::TicketDeleteFaiIdNotFound { .. } => {
				(StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
			},

			// -- Fallback
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR
			)
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	INVALID_PARAMS,
	SERVICE_ERROR
}