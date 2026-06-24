use sqlx::SqlitePool;
use uuid::Uuid;

pub struct TranscriptionRepository {
    pool: SqlitePool,
}

impl TranscriptionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, filename: &str) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO transcriptions (id, filename, status) VALUES (?, ?, ?)")
            .bind(id.to_string())
            .bind(filename)
            .bind("PENDING")
            .execute(&self.pool)
            .await?;
        
        Ok(id)
    }
}