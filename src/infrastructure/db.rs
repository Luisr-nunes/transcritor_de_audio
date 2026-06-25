use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

/// Cria o pool de conexão e garante que o schema existe.
pub async fn establish_connection() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://transcriptions.db".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados SQLite");

    // Cria tabela caso não exista (fallback; em produção use migrations SQLx)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transcriptions (
            id        TEXT    PRIMARY KEY,
            filename  TEXT    NOT NULL,
            status    TEXT    NOT NULL DEFAULT 'PENDING',
            content   TEXT
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Falha ao criar tabela transcriptions");

    info!("Banco de dados conectado: {database_url}");
    pool
}
