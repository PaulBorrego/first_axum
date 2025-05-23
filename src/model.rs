//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc,Mutex};

// Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: usize,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// Model Controller
#[derive(Clone)]
pub struct ModelController {
    //Clones arc not vec
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as usize;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: usize) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound {id})
    }

}