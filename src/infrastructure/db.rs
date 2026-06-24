use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn establish_connection() -> SqlitePool {
    let database_url = "sqlite://transcriptions.db";
    
    // Cria o pool de conexão
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    // Executa a migração (cria a tabela se não existir)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS transcriptions (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            status TEXT NOT NULL,
            content TEXT
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}