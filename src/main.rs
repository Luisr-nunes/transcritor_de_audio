mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;

use infrastructure::{
    api::build_router,
    db::establish_connection,
    repository::transcription::TranscriptionRepository,
};

#[tokio::main]
async fn main() {
    // Inicializa logs (RUST_LOG=info por padrão)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Conecta ao banco e garante o schema
    let pool = establish_connection().await;

    // Cria repositório compartilhado
    let repo = Arc::new(TranscriptionRepository::new(pool));

    // Constrói o roteador
    let app = build_router(repo);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Falha ao iniciar o listener TCP");

    info!("Servidor rodando em http://{addr}");
    info!("Endpoints disponíveis:");
    info!("  POST /upload              — envia áudio para transcrição");
    info!("  GET  /transcriptions      — lista todas as transcrições");
    info!("  GET  /transcriptions/:id  — consulta uma transcrição");

    axum::serve(listener, app)
        .await
        .expect("Falha ao servir a aplicação");
}
