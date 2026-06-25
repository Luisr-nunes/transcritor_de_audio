use std::sync::Arc;
use tracing::{error, info};

use crate::{
    domain::transcription::TranscriptionStatus,
    infrastructure::{
        repository::transcription::TranscriptionRepository,
        whisper::{decode_audio_to_samples, transcribe_audio, WhisperError},
    },
};

/// Caminho padrão para o modelo Whisper (pode ser sobrescrito via env var).
const DEFAULT_MODEL: &str = "assets/ggml-base.bin";

/// Erros do use case de transcrição.
#[derive(Debug, thiserror::Error)]
pub enum TranscribeError {
    #[error("Repositório: {0}")]
    Repository(String),

    #[error("Whisper: {0}")]
    Whisper(#[from] WhisperError),
}

/// Use case: recebe bytes de áudio, transcreve e persiste o resultado.
///
/// # Fluxo
/// 1. Cria registro no banco com status PENDING.
/// 2. Marca como PROCESSING.
/// 3. Decodifica áudio → samples PCM f32.
/// 4. Transcreve via Whisper.
/// 5. Persiste o texto e marca COMPLETED.
///    Em caso de erro, marca FAILED.
pub async fn transcribe(
    repo: Arc<TranscriptionRepository>,
    filename: String,
    audio_bytes: Vec<u8>,
    language: Option<String>,
    model_path: Option<String>,
) -> Result<String, TranscribeError> {
    // 1. Cria registro
    let transcription = repo
        .create(&filename)
        .await
        .map_err(|e| TranscribeError::Repository(e.to_string()))?;

    let id = transcription.id;
    info!("Transcrição criada: {id} — arquivo: {filename}");

    // 2. Marca como processing
    repo.update_status(id, TranscriptionStatus::Processing)
        .await
        .map_err(|e| TranscribeError::Repository(e.to_string()))?;

    // 3 & 4. Resolve caminho do modelo, decodifica e transcreve
    let model = model_path
        .as_deref()
        .unwrap_or(DEFAULT_MODEL);

    let result = (|| -> Result<String, TranscribeError> {
        let samples = decode_audio_to_samples(&audio_bytes)?;
        let text    = transcribe_audio(model, &samples, language.as_deref())?;
        Ok(text)
    })();

    match result {
        Ok(text) => {
            // 5. Persiste resultado
            repo.save_result(id, &text)
                .await
                .map_err(|e| TranscribeError::Repository(e.to_string()))?;
            info!("Transcrição {id} concluída.");
            Ok(text)
        }
        Err(e) => {
            error!("Transcrição {id} falhou: {e}");
            let _ = repo.mark_failed(id).await;
            Err(e)
        }
    }
}
