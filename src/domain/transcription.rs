use uuid::Uuid;

/// Representa o estado de uma transcrição no ciclo de vida do sistema.
#[derive(Debug, Clone, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum TranscriptionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for TranscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending    => write!(f, "PENDING"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::Completed  => write!(f, "COMPLETED"),
            Self::Failed     => write!(f, "FAILED"),
        }
    }
}

impl std::str::FromStr for TranscriptionStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PENDING"    => Ok(Self::Pending),
            "PROCESSING" => Ok(Self::Processing),
            "COMPLETED"  => Ok(Self::Completed),
            "FAILED"     => Ok(Self::Failed),
            other        => Err(format!("Status desconhecido: {other}")),
        }
    }
}

/// Entidade principal do domínio.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transcription {
    pub id:                Uuid,
    pub original_filename: String,
    pub status:            TranscriptionStatus,
    pub content:           Option<String>,
}

impl Transcription {
    pub fn new(original_filename: impl Into<String>) -> Self {
        Self {
            id:                Uuid::new_v4(),
            original_filename: original_filename.into(),
            status:            TranscriptionStatus::Pending,
            content:           None,
        }
    }
}
