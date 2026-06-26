use std::sync::Arc;
use tracing::{error, info};

use crate::{
    domain::transcription::TranscriptionStatus,
    infrastructure::{
        repository::transcription::TranscriptionRepository,
        whisper::{decode_audio_to_samples, transcribe_audio, WhisperError},
    },
};

/// Erros do use case de transcrição.
#[derive(Debug, thiserror::Error)]
pub enum TranscribeError {
    #[error("Repositório: {0}")]
    Repository(String),

    #[error("Whisper: {0}")]
    Whisper(#[from] WhisperError),
}

/// Resolve o caminho do modelo Whisper:
/// 1. Argumento explícito
/// 2. Variável de ambiente WHISPER_MODEL
/// 3. Pasta `assets/` relativa ao executável
/// 4. Pasta `assets/` relativa ao diretório de trabalho (fallback)
fn resolve_model_path(explicit: Option<&str>) -> String {
    if let Some(p) = explicit {
        return p.to_string();
    }
    if let Ok(env) = std::env::var("WHISPER_MODEL") {
        return env;
    }

    // Relativo ao executável (funciona em target/debug e release)
    if let Ok(exe) = std::env::current_exe() {
        let candidate = exe
            .parent()
            .map(|d| d.join("assets").join("ggml-base.bin"))
            .filter(|p| p.exists());
        if let Some(p) = candidate {
            return p.to_string_lossy().to_string();
        }
    }

    // Relativo ao diretório de trabalho (cargo run a partir da raiz do projeto)
    "assets/ggml-base.bin".to_string()
}

/// Use case: recebe bytes de áudio, transcreve e persiste o resultado.
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

    // 3. Resolve modelo
    let model = resolve_model_path(model_path.as_deref());
    info!("Usando modelo: {model}");

    // 4. Decodifica e transcreve (síncrono — roda na thread atual)
    let result = (|| -> Result<String, TranscribeError> {
        let samples = decode_audio_to_samples(&audio_bytes)?;
        let text    = transcribe_audio(&model, &samples, language.as_deref())?;
        Ok(text)
    })();

    match result {
        Ok(text) => {
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
