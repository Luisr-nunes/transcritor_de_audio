# Audio Transcriber

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![Tokio](https://img.shields.io/badge/Tokio-4A4A4A?style=flat-square) ![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white) ![Whisper](https://img.shields.io/badge/Whisper-412991?style=flat-square&logo=openai&logoColor=white) ![FFmpeg](https://img.shields.io/badge/FFmpeg-007808?style=flat-square&logo=ffmpeg&logoColor=white) ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white)

[Ler em Português](README.md)

A Rust audio transcription service that runs entirely on your machine: audio is processed locally by the Whisper model, with nothing sent to the cloud. It exposes a REST API and ships a web UI embedded in the binary.

## Features

- Audio upload through the web UI or `multipart/form-data`.
- WAV, MP3, MP4, OGG, FLAC and WebM; anything other than WAV is converted with `ffmpeg`.
- Native WAV decoding: 8, 16, 24 and 32-bit PCM, 32 and 64-bit float, mono and stereo.
- Automatic resampling to 16 kHz, as Whisper requires.
- Language: Portuguese, English, Spanish or auto-detect.
- SQLite history with per-transcription status: `PENDING`, `PROCESSING`, `COMPLETED` or `FAILED`.
- Web UI embedded with `include_str!`, no static files at runtime.

## Tech stack

| Layer | Technology |
| --- | --- |
| Web | Axum 0.7, Tokio, Tower |
| Database | SQLite via SQLx 0.7 |
| Transcription | Whisper.cpp via `whisper-rs` 0.16 |
| Audio conversion | `ffmpeg` (external process) |
| Errors and logs | `thiserror`, `tracing`, `tracing-subscriber` |

## Architecture

```
src/
├── main.rs                         # server startup (127.0.0.1:3000)
├── domain/transcription.rs         # Transcription entity and status
├── application/transcribe.rs       # transcription use case
├── infrastructure/
│   ├── api.rs                      # Axum routes
│   ├── db.rs                       # SQLite pool and schema
│   ├── whisper.rs                  # Whisper, WAV decoder and ffmpeg
│   ├── http/handler.rs             # upload, list and get
│   └── repository/transcription.rs # persistence
└── ui/index.html                   # embedded UI
migrations/0001_create_transcriptions.sql
```

## Requirements

- Rust 1.75+
- CMake and a C/C++ compiler, used by `whisper-rs` to build Whisper.cpp
- `ffmpeg` on PATH, for formats other than WAV
- Whisper model at `assets/ggml-base.bin` (about 140 MB, not versioned)

```bash
mkdir assets
curl -L -o assets/ggml-base.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

On Windows, `ffmpeg` can be installed with `winget install ffmpeg`.

## Running

```bash
git clone https://github.com/Luisr-nunes/transcritor_de_audio.git
cd transcritor_de_audio
cargo run            # or: make run
```

Open `http://127.0.0.1:3000`. For an optimized build, `cargo build --release` produces `target/release/transcription-service` (`.exe` on Windows).

The project manifest is saved as `cargo.toml`, in lowercase. That works on Windows, but on Linux and macOS Cargo requires `Cargo.toml`; rename the file before building there.

## API

| Method | Route | Description |
| --- | --- | --- |
| GET | `/` | Web UI |
| POST | `/upload` | Upload audio (`file` and `language` fields) |
| GET | `/transcriptions` | List transcriptions |
| GET | `/transcriptions/:id` | Get one transcription |

```bash
curl -X POST http://127.0.0.1:3000/upload -F "file=@audio.wav" -F "language=pt"
```

```json
{ "status": "completed", "transcription": "Pronto, pode falar, tá gravando." }
```

## Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://<executable folder>/transcriptions.db?mode=rwc` | SQLite database |
| `WHISPER_MODEL` | `assets/ggml-base.bin` | Model path |
| `RUST_LOG` | `info` | Log level |

## Author

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)
