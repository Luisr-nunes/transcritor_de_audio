use axum::{routing::{get, post}, Router};
use crate::infrastructure::http::handler::{
    AppState,
    get_transcription_handler,
    list_transcriptions_handler,
    upload_handler,
};

/// Constrói o roteador principal da aplicação.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/upload",              post(upload_handler))
        .route("/transcriptions",      get(list_transcriptions_handler))
        .route("/transcriptions/:id",  get(get_transcription_handler))
        .with_state(state)
}
