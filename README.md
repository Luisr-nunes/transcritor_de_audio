# 🎙️ transcritor_de_audio

Serviço de transcrição de áudio em Rust usando **Whisper.cpp** (via `whisper-rs`), **Axum** e **SQLite**.

## Stack

| Camada       | Tecnologia                    |
|--------------|-------------------------------|
| Web          | Axum 0.7 + Tokio              |
| Banco        | SQLite via SQLx               |
| Transcrição  | Whisper.cpp (`whisper-rs`)    |
| Logs         | tracing + tracing-subscriber  |

## Arquitetura

```
src/
├── main.rs
├── application/
│   └── transcribe.rs      # Use case principal
├── domain/
│   └── transcription.rs   # Entidade + Status
└── infrastructure/
    ├── api.rs             # Roteador Axum
    ├── db.rs              # Pool SQLite
    ├── whisper.rs         # Integração Whisper
    ├── http/
    │   └── handler.rs     # Handlers HTTP
    └── repository/
        └── transcription.rs  # CRUD
```

## Pré-requisitos

- Rust 1.75+
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp) compilado no sistema
- Modelo Whisper: `assets/ggml-base.bin`

```bash
# Baixar modelo base (~140 MB)
mkdir -p assets
wget -O assets/ggml-base.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

## Rodando

```bash
cargo run
# Servidor em http://127.0.0.1:3000
```

## Endpoints

| Método | Rota                    | Descrição                        |
|--------|-------------------------|----------------------------------|
| POST   | `/upload`               | Envia áudio para transcrição     |
| GET    | `/transcriptions`       | Lista todas as transcrições      |
| GET    | `/transcriptions/:id`   | Consulta transcrição por ID      |

### Exemplo de upload

```bash
curl -X POST http://127.0.0.1:3000/upload \
  -F "file=@audio.wav" \
  -F "language=pt"
```

### Resposta

```json
{
  "status": "completed",
  "transcription": "Olá, este é um teste de transcrição."
}
```

## Variáveis de ambiente

| Variável        | Padrão                     | Descrição                  |
|-----------------|----------------------------|----------------------------|
| `DATABASE_URL`  | `sqlite://transcriptions.db` | Caminho do banco SQLite    |
| `WHISPER_MODEL` | `assets/ggml-base.bin`     | Modelo Whisper a usar      |
| `RUST_LOG`      | `info`                     | Nível de log               |

## Notas

- O decodificador de áudio atual suporta **WAV PCM 16-bit mono 16 kHz**. Para outros formatos (MP3, MP4, OGG), integre `symphonia` ou pré-processe com `ffmpeg`.
- Para produção, considere processar a transcrição em background com `tokio::spawn`.
