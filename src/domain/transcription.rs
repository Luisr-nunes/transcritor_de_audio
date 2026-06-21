use uuid::Uuid;

pub struct Transcription {
    pub id: Uuid,
    pub original_filename: String,
    pub status: TranscriptionStatus,
    pub content: Option<String>,
}

pub enum TranscriptionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}