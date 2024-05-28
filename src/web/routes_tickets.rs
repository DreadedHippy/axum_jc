use axum::{extract::{FromRef, Path, State}, routing::{delete, post}, Json, Router};

use crate::{ctx::Ctx, models::{ModelController, Ticket,TicketForCreate}, ServerResult};

#[derive(Clone, FromRef)]
struct AppState {
	mc: ModelController
}

pub fn routes(mc: ModelController) -> Router {
	// let app_state = AppState {mc};

	Router::new()
		.route("/tickets", post(create_ticket).get(list_tickets))
		.route("/tickets/:id", delete(delete_ticket))
		.with_state(mc)
}

async fn create_ticket(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Json(ticket_fc): Json<TicketForCreate>
) -> ServerResult<Json<Ticket>> {
	println!("->> {:>12} - create ticket", "HANDLER");

	let ticket = mc.create_ticket(ctx, ticket_fc).await?;

	Ok(Json(ticket))
}

async fn list_tickets(
	State(mc): State<ModelController>,
	ctx: Ctx
) -> ServerResult<Json<Vec<Ticket>>> {
	println!("{:>12} - list_tickets", "HANDLER");

	let tickets = mc.list_tickets(ctx).await?;

	Ok(Json(tickets))
}

async fn delete_ticket(
	State(mc): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>
) -> ServerResult<Json<Ticket>> {
	println!("{:>12} - delete_ticket", "HANDLER");

	let ticket = mc.delete_ticket(ctx, id).await?;

	Ok(Json(ticket))

}
