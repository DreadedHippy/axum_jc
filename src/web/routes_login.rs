use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web::AUTH_TOKEN, ServerError, ServerResult};

pub fn routes() -> Router {
	Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> ServerResult<Json<Value>> {
	println!("->> {:12} - api_login", "HANDLER");

	// TODO: Implement real db/auth logic.
	if payload.username != "demo1" || payload.pwd != "welcome" {
		return Err(ServerError::LoginFail)
	}

	// FIXME: Implement real auth-token generation/signature
	cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));


	// Create the success body
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String
}