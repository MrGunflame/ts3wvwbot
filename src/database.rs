use sqlx::mysql::MySqlPool;
use sqlx::{query, Connection};

#[derive(Clone)]
pub(crate) struct DBHandler {
    pool: MySqlPool,
}

impl DBHandler {
    pub async fn new() -> Result<DBHandler, sqlx::Error> {
        Ok(DBHandler {
            pool: MySqlPool::connect("mysql://root:1234@172.17.0.2/ts3wvwbot").await?,
        })
    }

    async fn init() {
        let tables = vec![
            "CREATE TABLE users (id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT, ts_uid VARCHAR(64) NOT NULL, api_token VARCHAR(100) NOT NULL)"
        ];
    }

    async fn insert_user(&self, ts_uid: &str, api_token: &str) -> Result<(), sqlx::Error> {
        query("INSERT INTO users (ts_uid, api_token) VALUES (?, ?)")
            .bind(ts_uid)
            .bind(api_token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn insert_guild(&self, name: &str, channel: Option<usize>) {}

    async fn delete_guild(&self, name: &str) -> Result<(), sqlx::Error> {
        query("DELETE FROM guilds WHERE name = ?")
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
