mod infrastructure;
use axum::{routing::post, Router};
use std::sync::Arc;
use infrastructure::{db, repository::transcription::TranscriptionRepository, http::handlers};

#[tokio::main]
async fn main() {
    
    let pool = db::establish_connection().await;
    
    
    let repo = Arc::new(TranscriptionRepository::new(pool));

    
    let app = Router::new()
        .route("/upload", post(handlers::upload_handler))
        .with_state(repo);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Servidor rodando em http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}