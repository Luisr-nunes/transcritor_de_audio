use sqlx::SqlitePool;
use uuid::Uuid;
use crate::domain::transcription::{Transcription, TranscriptionStatus};

/// Row de banco — facilita o mapeamento sqlx sem exigir derives no domínio.
#[derive(sqlx::FromRow)]
struct TranscriptionRow {
    id:       String,
    filename: String,
    status:   String,
    content:  Option<String>,
}

impl TryFrom<TranscriptionRow> for Transcription {
    type Error = String;

    fn try_from(row: TranscriptionRow) -> Result<Self, Self::Error> {
        let id     = Uuid::parse_str(&row.id).map_err(|e| e.to_string())?;
        let status = row.status.parse::<TranscriptionStatus>()?;
        Ok(Transcription { id, original_filename: row.filename, status, content: row.content })
    }
}

/// Repositório de persistência de transcrições.
#[derive(Clone)]
pub struct TranscriptionRepository {
    pool: SqlitePool,
}

impl TranscriptionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Insere uma nova transcrição com status PENDING.
    pub async fn create(&self, filename: &str) -> Result<Transcription, sqlx::Error> {
        let t = Transcription::new(filename);
        sqlx::query(
            "INSERT INTO transcriptions (id, filename, status) VALUES (?, ?, ?)",
        )
        .bind(t.id.to_string())
        .bind(&t.original_filename)
        .bind(t.status.to_string())
        .execute(&self.pool)
        .await?;
        Ok(t)
    }

    /// Atualiza o status de uma transcrição.
    pub async fn update_status(
        &self,
        id: Uuid,
        status: TranscriptionStatus,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE transcriptions SET status = ? WHERE id = ?")
            .bind(status.to_string())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Persiste o texto transcrito e marca como COMPLETED.
    pub async fn save_result(&self, id: Uuid, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE transcriptions SET status = ?, content = ? WHERE id = ?",
        )
        .bind(TranscriptionStatus::Completed.to_string())
        .bind(content)
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Marca como FAILED.
    pub async fn mark_failed(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE transcriptions SET status = ? WHERE id = ?")
            .bind(TranscriptionStatus::Failed.to_string())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Busca por ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Transcription>, String> {
        let row: Option<TranscriptionRow> =
            sqlx::query_as("SELECT id, filename, status, content FROM transcriptions WHERE id = ?")
                .bind(id.to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        row.map(Transcription::try_from).transpose()
    }

    /// Lista todas as transcrições.
    pub async fn list_all(&self) -> Result<Vec<Transcription>, String> {
        let rows: Vec<TranscriptionRow> =
            sqlx::query_as("SELECT id, filename, status, content FROM transcriptions ORDER BY rowid DESC")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        rows.into_iter().map(Transcription::try_from).collect()
    }
}
