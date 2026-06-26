use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

/// Cria o pool de conexão e garante que o schema existe.
pub async fn establish_connection() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Resolve o caminho do banco relativo ao executável,
        // garantindo que o SQLite consiga criar o arquivo.
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let db_path = exe_dir.join("transcriptions.db");

        // SQLite exige "sqlite://" + caminho absoluto com barras normais
        // e o parâmetro "?mode=rwc" para criar o arquivo se não existir.
        let path_str = db_path
            .to_string_lossy()
            .replace('\\', "/");

        format!("sqlite://{}?mode=rwc", path_str)
    });

    info!("Conectando ao banco: {database_url}");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|e| panic!("Falha ao conectar ao banco de dados SQLite: {e}"));

    // Cria tabela caso não exista
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transcriptions (
            id         TEXT PRIMARY KEY,
            filename   TEXT NOT NULL,
            status     TEXT NOT NULL DEFAULT 'PENDING',
            content    TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Falha ao criar tabela transcriptions");

    info!("Banco de dados pronto.");
    pool
}
