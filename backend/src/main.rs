mod create_ticket_handler;
mod error;
mod storage;

use axum::routing::post;
use dotenv::dotenv;

use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    dotenv().ok();
    let storage = storage::Storage::connect().await?;
    let shared_storage = Arc::new(tokio::sync::Mutex::new(storage));
    let app = axum::Router::new()
        .route(
            "/ticket",
            post(create_ticket_handler::create_ticket_handler),
        )
        .with_state(shared_storage);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
