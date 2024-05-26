//! Simplistic model layer
//! (with mock-store layer)

use crate::{ServerError, ServerResult};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

// region: Ticket Types

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
	pub id: u64,
	pub title: String
}

#[derive(Clone, Deserialize)]
pub struct TicketForCreate {
	pub title: String
}
// endregion

// region: Model controller

#[derive(Clone)]
pub struct ModelController {
	tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
	pub async fn new() -> ServerResult<Self> {
		Ok(
			Self{
				tickets_store: Arc::default()
			}
		)
	}
}

// CRUD Implementation
impl ModelController {
	pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> ServerResult<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let id = store.len() as u64;
		let ticket = Ticket {
			id,
			title: ticket_fc.title
		};

		store.push(Some(ticket.clone()));

		Ok(ticket)
	}

	pub async fn list_tickets(&self) -> ServerResult<Vec<Ticket>> {
		let store = self.tickets_store.lock().unwrap();

		let tickets = store.iter().filter_map(|t| t.clone()).collect();

		Ok(tickets)
	}

	pub async fn delete_ticket(&self, id: u64) -> ServerResult<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let ticket = store.get_mut(id as usize).and_then(|t| t.take());

		ticket.ok_or(ServerError::TicketDeleteFaiIdNotFound {id})
	}
}
// endregion
