use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use tracing::error;

use crate::{
    application::transcribe::transcribe,
    infrastructure::repository::transcription::TranscriptionRepository,
};

pub type AppState = Arc<TranscriptionRepository>;

/// POST /upload
pub async fn upload_handler(
    State(repo): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut audio_bytes: Option<Vec<u8>> = None;
    let mut filename = "audio".to_string();
    let mut language: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name().unwrap_or("") {
            "file" => {
                filename = field.file_name().unwrap_or("audio.wav").to_string();
                match field.bytes().await {
                    Ok(b) => audio_bytes = Some(b.to_vec()),
                    Err(e) => {
                        error!("Falha ao ler bytes do campo 'file': {e}");
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({ "error": format!("Falha ao ler arquivo: {e}") })),
                        );
                    }
                }
            }
            "language" => {
                language = field.text().await.ok();
            }
            _ => {}
        }
    }

    let bytes = match audio_bytes {
        Some(b) if !b.is_empty() => b,
        Some(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Arquivo enviado está vazio." })),
            );
        }
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Campo 'file' não encontrado no formulário." })),
            );
        }
    };

    match transcribe(repo, filename, bytes, language, None).await {
        Ok(text) => (
            StatusCode::OK,
            Json(json!({ "status": "completed", "transcription": text })),
        ),
        Err(e) => {
            error!("Erro na transcrição: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "failed", "error": e.to_string() })),
            )
        }
    }
}

/// GET /transcriptions/:id
pub async fn get_transcription_handler(
    State(repo): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "ID inválido." })),
            );
        }
    };

    match repo.find_by_id(uuid).await {
        Ok(Some(t)) => (
            StatusCode::OK,
            Json(json!({
                "id":       t.id,
                "filename": t.original_filename,
                "status":   t.status.to_string(),
                "content":  t.content,
            })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Transcrição não encontrada." })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        ),
    }
}

/// GET /transcriptions
pub async fn list_transcriptions_handler(
    State(repo): State<AppState>,
) -> impl IntoResponse {
    match repo.list_all().await {
        Ok(list) => {
            let body: Vec<_> = list
                .iter()
                .map(|t| json!({
                    "id":       t.id,
                    "filename": t.original_filename,
                    "status":   t.status.to_string(),
                    "content":  t.content,
                }))
                .collect();
            (StatusCode::OK, Json(json!(body)))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        ),
    }
}
