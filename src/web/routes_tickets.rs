use axum::{extract::{FromRef, Path, State}, routing::{delete, get, post}, Json, Router};

use crate::{models::{ModelController, Ticket,TicketForCreate}, ServerResult};

#[derive(Clone, FromRef)]
struct AppState {
	mc: ModelController
}

pub fn routes(mc: ModelController) -> Router {
	// let app_state = AppState {mc};

	Router::new()
		.route("/tickets", post(create_ticket).get(list_tickets))
		.route("/ticket: id", delete(delete_ticket))
		.with_state(mc)
}

async fn create_ticket(
	State(mc): State<ModelController>,
	Json(ticket_fc): Json<TicketForCreate>
) -> ServerResult<Json<Ticket>> {
	println!("->> {:>12} - create ticket", "HANDLER");

	let ticket = mc.create_ticket(ticket_fc).await?;

	Ok(Json(ticket))
}

async fn list_tickets(
	State(mc): State<ModelController>
) -> ServerResult<Json<Vec<Ticket>>> {
	println!("{:>12} - list_tickets", "HANDLER");

	let tickets = mc.list_tickets().await?;

	Ok(Json(tickets))
}

async fn delete_ticket(
	State(mc): State<ModelController>,
	Path(id): Path<u64>
) -> ServerResult<Json<Ticket>> {
	println!("{:>12} - delete_ticket", "HANDLER");

	let ticket = mc.delete_ticket(id).await?;

	Ok(Json(ticket))

}
