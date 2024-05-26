use axum::{http::StatusCode, response::IntoResponse};

pub type ServerResult<T> = core::result::Result<T, ServerError>;

#[derive(Debug)]
pub enum ServerError {
	LoginFail
}

impl IntoResponse for ServerError {
	fn into_response(self) -> axum::response::Response {
		println!("{:<12} - {self:?}", "INTO_RES");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT ERROR").into_response()
	}
}