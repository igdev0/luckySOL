use sqlx::{migrate::MigrateDatabase, Connection, MySql, MySqlConnection};
use std::{env, error::Error, ops::Deref};

#[derive(Debug)]
pub struct Storage(Box<MySqlConnection>);

impl Storage {
    pub async fn connect() -> Result<Self, Box<dyn Error>> {
        let url = Self::get_db_url()?;
        let connection = MySql::database_exists(&url).await?;

        if !connection {
            MySql::create_database(&url).await?;
        }
        let connection = MySqlConnection::connect(&url).await?;

        Ok(Self(Box::new(connection)))
    }

    pub async fn migrate(&mut self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!().run(&mut *self.0).await
    }

    fn get_db_url() -> Result<String, Box<dyn Error>> {
        let db_host = env::var("DB_HOST")?;
        let db_name = env::var("DB_NAME")?;
        let db_username = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_port: u64 = env::var("DB_PORT")?.parse()?;
        Ok(format!(
            "mysql://{}:{}@{}:{}/{}",
            db_username, db_password, db_host, db_port, db_name
        )
        .to_string())
    }
}

impl Deref for Storage {
    type Target = MySqlConnection;
    fn deref(&self) -> &Self::Target {
        &&self.0
    }
}
