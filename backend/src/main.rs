mod error;
mod storage;

use dotenv::dotenv;
use std::{error::Error, ops::Deref};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    dotenv().ok();
    let _connection = storage::Storage::connect().await?.deref();

    Ok(())
}
