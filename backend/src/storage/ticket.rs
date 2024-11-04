use std::ops::DerefMut;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{mysql::MySqlRow, MySqlConnection};

use super::Storage;

pub type LuckyDraft = [u8; 6];

#[derive(Debug, Deserialize, Serialize)]
pub enum TicketStatus {
    PendingCreation,
    Created,
    Win,
    Loss,
}

#[derive(Deserialize, Serialize)]
pub struct TicketEntity {
    pub id: String,
    pub address: String,
    pub merkle_hash: String,
    pub status: TicketStatus,
    pub lucky_draft: Vec<LuckyDraft>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct SaveTicketResponse {
    pub merkle_root: Vec<u8>,
}

impl std::fmt::Display for TicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            TicketStatus::PendingCreation => "PendingCreation",
            TicketStatus::Created => "Created",
            TicketStatus::Win => "Win",
            TicketStatus::Loss => "Loss",
        };
        write!(f, "{}", status)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketInput {
    pub address: String,
    pub lucky_draft: Vec<LuckyDraft>,
}

impl CreateTicketInput {
    pub fn new(address: String, lucky_draft: Vec<LuckyDraft>) -> Self {
        Self {
            address,
            lucky_draft,
        }
    }

    async fn find_previous_tickets(
        &self,
        storage: &mut Storage,
    ) -> Result<Vec<MySqlRow>, sqlx::Error> {
        let mysql_connection = storage.0.deref_mut();
        sqlx::query("SELECT * FROM WHERE address = ?;")
            .bind(self.address.clone())
            .fetch_all(mysql_connection)
            .await
    }

    fn create_merkle_hash(&self) -> String {
        String::new()
    }

    fn get_merkle_root(&self) -> Vec<u8> {
        Vec::new()
    }

    pub async fn save(
        &self,
        storage: &mut MySqlConnection,
    ) -> Result<SaveTicketResponse, sqlx::Error> {
        let query = sqlx::query("INSERT INTO tickets (id, address, merkle_hash, lucky_draft, status) values (?, ?, ?, ?, ?);");
        let id = uuid::Uuid::new_v4();
        let merkle_hash = self.create_merkle_hash();
        let merkle_root = self.get_merkle_root();

        let result = query
            .bind(id.to_string())
            .bind(self.address.clone())
            .bind(merkle_hash)
            .bind(json!(self.lucky_draft.clone()))
            .bind(TicketStatus::PendingCreation.to_string())
            .execute(storage)
            .await;

        if let Err(e) = result {
            return Err(e);
        }

        Ok(SaveTicketResponse { merkle_root })
    }
}
