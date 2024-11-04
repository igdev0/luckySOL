use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::storage::{
    ticket::{CreateTicketInput, SaveTicketResponse, TicketStatus},
    Storage,
};

#[axum_macros::debug_handler]
pub async fn create_ticket_handler(
    State(storage): State<Arc<tokio::sync::Mutex<Storage>>>,
    Json(ticket): Json<CreateTicketInput>,
) -> (StatusCode, Json<SaveTicketResponse>) {
    let lucky_draft = ticket.lucky_draft.clone();
    let address = ticket.address.clone();

    let mut cloned_storage = storage.deref().lock().await;
    let new_storage = cloned_storage.deref_mut();
    let input = CreateTicketInput::new(address.clone(), lucky_draft);
    let st = &mut **new_storage;

    let response = input.save(st).await;

    match response {
        Ok(res) => (StatusCode::OK, axum::Json(res)),
        Err(_e) => (
            StatusCode::NOT_ACCEPTABLE,
            axum::Json(SaveTicketResponse {
                merkle_root: Vec::new(),
            }),
        ),
    }
}
