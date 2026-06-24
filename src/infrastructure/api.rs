use axum::{
    extract::{Multipart, State},
    routing::post,
    Router,
};
use std::sync::Arc;

pub async fn app() -> Router {
    Router::new()
        .route("/upload", post(upload_handler))
}

async fn upload_handler(mut multipart: Multipart) -> &'static str {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        
        println!("Recebido arquivo: {} com {} bytes", name, data.len());
    
    }
    "Arquivo recebido com sucesso!"
}