use std::path::Path;
use tracing::info;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Erros possíveis durante a transcrição via Whisper.
#[derive(Debug, thiserror::Error)]
pub enum WhisperError {
    #[error("Falha ao carregar o modelo Whisper: {0}")]
    ModelLoad(String),

    #[error("Falha ao criar estado de processamento: {0}")]
    StateCreate(String),

    #[error("Falha ao processar o áudio: {0}")]
    Processing(String),

    #[error("Arquivo de modelo não encontrado: {0}")]
    ModelNotFound(String),

    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),
}

/// Transcreve amostras de áudio PCM f32 usando um modelo Whisper local.
///
/// # Argumentos
/// * `model_path` - Caminho para o arquivo `.bin` do modelo (ex: `assets/ggml-base.bin`)
/// * `samples`    - Amostras PCM 16 kHz mono em f32 normalizado entre -1.0 e 1.0
/// * `language`   - Código de idioma ISO 639-1 (ex: `"pt"`, `"en"`). `None` = autodetect.
pub fn transcribe_audio(
    model_path: &str,
    samples: &[f32],
    language: Option<&str>,
) -> Result<String, WhisperError> {
    if !Path::new(model_path).exists() {
        return Err(WhisperError::ModelNotFound(model_path.to_string()));
    }

    info!("Carregando modelo Whisper: {model_path}");

    let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
        .map_err(|e| WhisperError::ModelLoad(e.to_string()))?;

    let mut state = ctx
        .create_state()
        .map_err(|e| WhisperError::StateCreate(e.to_string()))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    params.set_n_threads(num_cpus());
    params.set_translate(false);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    if let Some(lang) = language {
        params.set_language(Some(lang));
    }

    info!("Processando {} amostras de áudio…", samples.len());

    state
        .full(params, samples)
        .map_err(|e| WhisperError::Processing(e.to_string()))?;

    // whisper-rs 0.13+: full_n_segments() retorna i32 diretamente (sem Result)
    let num_segments = state.full_n_segments()
        .map_err(|e| WhisperError::Processing(e.to_string()))?;

    let mut result = String::new();
    for i in 0..num_segments {
        // whisper-rs 0.13+: get_segment(i) substituiu full_get_segment_text(i)
        let segment = state
            .get_segment(i)
            .ok_or_else(|| WhisperError::Processing(format!("Segmento {i} não encontrado")))?;
        result.push_str(segment.trim());
        result.push(' ');
    }

    let text = result.trim().to_string();
    info!("Transcrição concluída ({} caracteres).", text.len());
    Ok(text)
}

/// Decodifica bytes WAV PCM 16-bit LE 16 kHz mono para samples f32.
/// Para outros formatos, pré-processe com `ffmpeg` antes de enviar.
pub fn decode_audio_to_samples(audio_bytes: &[u8]) -> Result<Vec<f32>, WhisperError> {
    if audio_bytes.len() < 44 {
        return Err(WhisperError::Processing(
            "Arquivo de áudio muito pequeno ou inválido".to_string(),
        ));
    }

    // Pula o cabeçalho WAV padrão de 44 bytes
    let pcm_bytes = &audio_bytes[44..];
    let samples: Vec<f32> = pcm_bytes
        .chunks_exact(2)
        .map(|chunk| {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            sample as f32 / i16::MAX as f32
        })
        .collect();

    Ok(samples)
}

/// Número de threads disponíveis, limitado a 4.
fn num_cpus() -> i32 {
    std::thread::available_parallelism()
        .map(|n| (n.get() as i32).min(4))
        .unwrap_or(2)
}
