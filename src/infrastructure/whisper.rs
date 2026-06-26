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

    #[error("Formato de áudio inválido ou não suportado: {0}")]
    InvalidAudio(String),

    #[error("ffmpeg não encontrado. Instale o ffmpeg para suporte a MP3/MP4/OGG: {0}")]
    FfmpegNotFound(String),

    #[error("Erro de I/O: {0}")]
    Io(#[from] std::io::Error),
}

/// Transcreve amostras de áudio PCM f32 usando um modelo Whisper local.
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

    info!(
        "Processando {} amostras ({:.1}s de áudio)…",
        samples.len(),
        samples.len() as f32 / 16000.0
    );

    state
        .full(params, samples)
        .map_err(|e| WhisperError::Processing(e.to_string()))?;

    let mut result = String::new();
    for segment in state.as_iter() {
        let text = segment.to_string();
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            result.push_str(trimmed);
            result.push(' ');
        }
    }

    let text = result.trim().to_string();
    info!("Transcrição concluída ({} caracteres).", text.len());
    Ok(text)
}

/// Detecta o formato do arquivo pelos magic bytes.
fn detect_format(bytes: &[u8]) -> &'static str {
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
        return "wav";
    }
    if bytes.len() >= 3 && &bytes[0..3] == b"ID3" {
        return "mp3";
    }
    if bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0 {
        return "mp3";
    }
    if bytes.len() >= 4 && &bytes[0..4] == b"OggS" {
        return "ogg";
    }
    if bytes.len() >= 8 && (&bytes[4..8] == b"ftyp" || &bytes[4..8] == b"moov") {
        return "mp4";
    }
    if bytes.len() >= 4 && &bytes[0..4] == b"fLaC" {
        return "flac";
    }
    if bytes.len() >= 4 && &bytes[0..4] == b"weba" {
        return "webm";
    }
    // WebM/MKV
    if bytes.len() >= 4 && bytes[0] == 0x1A && bytes[1] == 0x45 && bytes[2] == 0xDF && bytes[3] == 0xA3 {
        return "webm";
    }
    "unknown"
}

/// Converte qualquer formato de áudio para WAV PCM 16 kHz mono via ffmpeg.
fn convert_with_ffmpeg(audio_bytes: &[u8], input_ext: &str) -> Result<Vec<u8>, WhisperError> {
    use std::io::Write;

    // Escreve o input em arquivo temporário
    let tmp_dir = std::env::temp_dir();
    let input_path  = tmp_dir.join(format!("transcritor_input.{input_ext}"));
    let output_path = tmp_dir.join("transcritor_output.wav");

    {
        let mut f = std::fs::File::create(&input_path)?;
        f.write_all(audio_bytes)?;
    }

    // Remove output anterior se existir
    let _ = std::fs::remove_file(&output_path);

    // Executa ffmpeg: converte para WAV PCM 16 kHz mono
    let status = std::process::Command::new("ffmpeg")
        .args([
            "-y",                                    // sobrescreve sem perguntar
            "-i", input_path.to_str().unwrap(),      // input
            "-ar", "16000",                          // sample rate 16 kHz
            "-ac", "1",                              // mono
            "-c:a", "pcm_s16le",                     // PCM 16-bit little-endian
            output_path.to_str().unwrap(),           // output
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| WhisperError::FfmpegNotFound(e.to_string()))?;

    let _ = std::fs::remove_file(&input_path);

    if !status.success() {
        return Err(WhisperError::InvalidAudio(
            "ffmpeg falhou ao converter o arquivo de áudio".to_string(),
        ));
    }

    let wav_bytes = std::fs::read(&output_path)?;
    let _ = std::fs::remove_file(&output_path);

    Ok(wav_bytes)
}

/// Decodifica qualquer arquivo de áudio para samples PCM f32 a 16 kHz mono.
/// Suporta WAV nativamente; outros formatos requerem ffmpeg instalado.
pub fn decode_audio_to_samples(audio_bytes: &[u8]) -> Result<Vec<f32>, WhisperError> {
    let format = detect_format(audio_bytes);
    info!("Formato de áudio detectado: {format}");

    // Se não for WAV, converte com ffmpeg primeiro
    let wav_bytes = if format == "wav" {
        audio_bytes.to_vec()
    } else {
        info!("Convertendo {format} → WAV via ffmpeg…");
        convert_with_ffmpeg(audio_bytes, format)?
    };

    decode_wav(&wav_bytes)
}

/// Decodifica bytes WAV para samples PCM f32 a 16 kHz mono.
fn decode_wav(audio_bytes: &[u8]) -> Result<Vec<f32>, WhisperError> {
    if audio_bytes.len() < 44 {
        return Err(WhisperError::InvalidAudio("Arquivo WAV muito pequeno".to_string()));
    }
    if &audio_bytes[0..4] != b"RIFF" || &audio_bytes[8..12] != b"WAVE" {
        return Err(WhisperError::InvalidAudio("Magic RIFF/WAVE ausente".to_string()));
    }

    // Percorre chunks para encontrar "fmt " e "data"
    let mut fmt_chunk:  Option<&[u8]> = None;
    let mut data_chunk: Option<&[u8]> = None;

    let mut pos = 12usize;
    while pos + 8 <= audio_bytes.len() {
        let tag   = &audio_bytes[pos..pos + 4];
        let size  = u32::from_le_bytes(audio_bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
        let start = pos + 8;
        let end   = (start + size).min(audio_bytes.len());

        match tag {
            b"fmt " => fmt_chunk  = Some(&audio_bytes[start..end]),
            b"data" => data_chunk = Some(&audio_bytes[start..end]),
            _ => {}
        }

        pos = start + size + (size % 2);
        if fmt_chunk.is_some() && data_chunk.is_some() {
            break;
        }
    }

    let fmt  = fmt_chunk .ok_or_else(|| WhisperError::InvalidAudio("Chunk 'fmt ' não encontrado".to_string()))?;
    let data = data_chunk.ok_or_else(|| WhisperError::InvalidAudio("Chunk 'data' não encontrado".to_string()))?;

    if fmt.len() < 16 {
        return Err(WhisperError::InvalidAudio("Chunk fmt muito pequeno".to_string()));
    }

    let audio_format    = u16::from_le_bytes([fmt[0],  fmt[1]]);
    let num_channels    = u16::from_le_bytes([fmt[2],  fmt[3]]);
    let sample_rate     = u32::from_le_bytes([fmt[4],  fmt[5],  fmt[6],  fmt[7]]);
    let bits_per_sample = u16::from_le_bytes([fmt[14], fmt[15]]);

    info!("WAV: format={audio_format} channels={num_channels} sample_rate={sample_rate} bits={bits_per_sample}");

    if num_channels == 0 {
        return Err(WhisperError::InvalidAudio("Número de canais inválido".to_string()));
    }

    // Converte para f32
    let samples_raw: Vec<f32> = match (audio_format, bits_per_sample) {
        (1, 8)  => data.iter().map(|&b| (b as f32 - 128.0) / 128.0).collect(),
        (1, 16) => data.chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / i16::MAX as f32)
            .collect(),
        (1, 24) => data.chunks_exact(3)
            .map(|c| {
                let sign = if c[2] & 0x80 != 0 { 0xFF } else { 0x00 };
                let v = i32::from_le_bytes([c[0], c[1], c[2], sign]);
                v as f32 / 8_388_608.0
            })
            .collect(),
        (1, 32) => data.chunks_exact(4)
            .map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]]) as f32 / i32::MAX as f32)
            .collect(),
        (3, 32) => data.chunks_exact(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect(),
        (3, 64) => data.chunks_exact(8)
            .map(|c| f64::from_le_bytes(c.try_into().unwrap()) as f32)
            .collect(),
        _ => return Err(WhisperError::InvalidAudio(
            format!("Formato WAV não suportado: format={audio_format} bits={bits_per_sample}")
        )),
    };

    // Mixdown para mono
    let ch = num_channels as usize;
    let mono: Vec<f32> = if ch == 1 {
        samples_raw
    } else {
        samples_raw.chunks_exact(ch)
            .map(|frame| frame.iter().sum::<f32>() / ch as f32)
            .collect()
    };

    // Reamostragem linear para 16 kHz
    let target = 16_000u32;
    let resampled = if sample_rate == target {
        mono
    } else {
        info!("Reamostrando {sample_rate} Hz → {target} Hz…");
        let ratio   = sample_rate as f64 / target as f64;
        let out_len = (mono.len() as f64 / ratio) as usize;
        (0..out_len)
            .map(|i| {
                let pos  = i as f64 * ratio;
                let idx  = pos as usize;
                let frac = (pos - idx as f64) as f32;
                let a = mono.get(idx    ).copied().unwrap_or(0.0);
                let b = mono.get(idx + 1).copied().unwrap_or(0.0);
                a + (b - a) * frac
            })
            .collect()
    };

    if resampled.is_empty() {
        return Err(WhisperError::InvalidAudio("Áudio vazio após decodificação".to_string()));
    }

    Ok(resampled)
}

/// Número de threads disponíveis, limitado a 4.
fn num_cpus() -> i32 {
    std::thread::available_parallelism()
        .map(|n| (n.get() as i32).min(4))
        .unwrap_or(2)
}
