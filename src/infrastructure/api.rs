use axum::{
    extract::DefaultBodyLimit,
    response::Html,
    routing::{get, post},
    Router,
};
use crate::infrastructure::http::handler::{
    AppState,
    get_transcription_handler,
    list_transcriptions_handler,
    upload_handler,
};

const UI_HTML: &str = include_str!("../ui/index.html");

async fn index_handler() -> Html<&'static str> {
    Html(UI_HTML)
}

/// Constrói o roteador principal da aplicação.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/upload", post(upload_handler))
        .route("/transcriptions", get(list_transcriptions_handler))
        .route("/transcriptions/:id", get(get_transcription_handler))
        // Permite uploads de até 500 MB
        .layer(DefaultBodyLimit::max(500 * 1024 * 1024))
        .with_state(state)
}
