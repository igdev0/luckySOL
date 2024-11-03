use sqlx::{migrate::MigrateDatabase, pool::PoolConnection, MySql};
use std::{env, error::Error};
pub struct Config {
    pub db_connection: PoolConnection<MySql>,
}

impl Config {}
