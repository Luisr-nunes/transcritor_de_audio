# 🎙️ Transcritor de Áudio

**PT** | [EN](#english-version)

> Serviço de transcrição de áudio via API REST — processamento local com Whisper, sem dependência de nuvem.

[![Version](https://img.shields.io/badge/version-0.1.0-orange?style=flat-square)](https://github.com/Luisr-nunes/transcritor_de_audio/releases)
[![Platform](https://img.shields.io/badge/platform-Windows-blue?style=flat-square&logo=windows)](https://github.com/Luisr-nunes/transcritor_de_audio)
[![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)](https://github.com/Luisr-nunes/transcritor_de_audio/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-backend-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/Axum-web%20framework-7c6af7?style=flat-square)](https://github.com/tokio-rs/axum)

---

## Sobre o Projeto

O **Transcritor de Áudio** é um serviço de transcrição que roda inteiramente na sua máquina. O áudio é processado localmente pelo modelo **Whisper** (via `whisper-rs`), sem enviar dados para servidores externos.

Expõe uma API REST construída com **Axum** e **Tokio**, persiste os resultados em **SQLite** via `SQLx`, e inclui uma interface web embutida no próprio binário para facilitar o uso sem necessidade de clientes externos.

---

## Funcionalidades

- Upload de áudio via interface web ou requisição `multipart/form-data`
- Suporte a múltiplos formatos: **WAV, MP3, MP4, OGG, FLAC, WebM** (conversão automática via `ffmpeg`)
- Decodificação WAV nativa: PCM 8, 16, 24 e 32 bits, IEEE Float 32/64 bits, mono e estéreo
- Reamostragem automática para 16 kHz (requisito do Whisper)
- Seleção de idioma: Português, Inglês, Espanhol ou autodetecção
- Persistência de todas as transcrições em banco SQLite local
- Histórico completo com status por transcrição (PENDING → PROCESSING → COMPLETED / FAILED)
- Interface web embutida no binário — sem dependências de arquivos estáticos em runtime

---

## Tecnologias Utilizadas

| Camada | Tecnologia | Descrição |
|---|---|---|
| Web Framework | **Axum 0.7** | Framework HTTP assíncrono baseado em Tokio |
| Runtime Async | **Tokio** | Runtime assíncrono para Rust |
| Banco de Dados | **SQLite** (SQLx) | Armazenamento local com pool de conexões |
| Transcrição | **Whisper.cpp** (`whisper-rs`) | Modelo de reconhecimento de fala local |
| Tratamento de Erros | **thiserror** | Derivação ergonômica de tipos de erro |
| Logs | **tracing** + **tracing-subscriber** | Logs estruturados com filtro por nível |
| Utilitários | **uuid** | Geração de IDs únicos por transcrição |

---

## Pré-requisitos

- [Rust](https://rustup.rs) 1.75+
- [ffmpeg](https://ffmpeg.org/download.html) instalado e no PATH (para formatos além de WAV)
- Modelo Whisper: `assets/ggml-base.bin`

```powershell
# Instalar ffmpeg via winget (Windows)
winget install ffmpeg

# Baixar o modelo Whisper base (~140 MB)
mkdir assets
curl -L -o assets/ggml-base.bin `
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

---

## Como Rodar

```powershell
# Clone o repositório
git clone https://github.com/Luisr-nunes/transcritor_de_audio.git
cd transcritor_de_audio

# Execute
cargo run
```

> O servidor sobe em `http://127.0.0.1:3000`. Acesse pelo navegador para usar a interface web.

### Build de Produção

```powershell
cargo build --release
# Executável em target/release/transcription-service.exe
```

---

## API

| Método | Rota | Descrição |
|---|---|---|
| `GET` | `/` | Interface web embutida |
| `POST` | `/upload` | Envia áudio para transcrição |
| `GET` | `/transcriptions` | Lista todas as transcrições |
| `GET` | `/transcriptions/:id` | Consulta transcrição por ID |

### Exemplo de upload via curl

```bash
curl -X POST http://127.0.0.1:3000/upload \
  -F "file=@audio.wav" \
  -F "language=pt"
```

### Resposta

```json
{
  "status": "completed",
  "transcription": "Pronto, pode falar, tá gravando."
}
```

---

## Variáveis de Ambiente

| Variável | Padrão | Descrição |
|---|---|---|
| `DATABASE_URL` | `sqlite://<exe_dir>/transcriptions.db?mode=rwc` | Caminho do banco SQLite |
| `WHISPER_MODEL` | `assets/ggml-base.bin` | Caminho do modelo Whisper |
| `RUST_LOG` | `info` | Nível de log (`debug`, `info`, `warn`, `error`) |

---

## Estrutura do Projeto

```
transcritor_de_audio/
├── src/
│   ├── main.rs                          # Entry point, inicialização do servidor
│   ├── application/
│   │   └── transcribe.rs               # Use case principal (fluxo de transcrição)
│   ├── domain/
│   │   └── transcription.rs            # Entidade Transcription + TranscriptionStatus
│   ├── infrastructure/
│   │   ├── api.rs                       # Roteador Axum com todas as rotas
│   │   ├── db.rs                        # Pool SQLite e criação do schema
│   │   ├── whisper.rs                   # Integração Whisper + decoder de áudio
│   │   ├── http/
│   │   │   └── handler.rs              # Handlers HTTP (upload, list, get)
│   │   └── repository/
│   │       └── transcription.rs        # CRUD de transcrições (SQLx)
│   └── ui/
│       └── index.html                  # Interface web embutida via include_str!
├── assets/
│   └── ggml-base.bin                   # Modelo Whisper (não versionado)
├── migrations/
│   └── 0001_create_transcriptions.sql  # Schema SQL versionado
└── Cargo.toml                          # Dependências Rust
```

---

## Desenvolvido por

**Luis Nunes** — [@Luisr-nunes](https://github.com/Luisr-nunes)

---

---

# 🎙️ Audio Transcriber

[PT](#) | **EN**

> Audio transcription service via REST API — local processing with Whisper, no cloud dependency.

---

## About

**Audio Transcriber** is a transcription service that runs entirely on your machine. Audio is processed locally by the **Whisper** model (via `whisper-rs`), with no data sent to external servers.

It exposes a REST API built with **Axum** and **Tokio**, persists results in **SQLite** via `SQLx`, and includes a web interface embedded in the binary itself — no external clients required.

---

## Features

- Audio upload via web interface or `multipart/form-data` request
- Multiple format support: **WAV, MP3, MP4, OGG, FLAC, WebM** (automatic conversion via `ffmpeg`)
- Native WAV decoding: PCM 8, 16, 24 and 32-bit, IEEE Float 32/64-bit, mono and stereo
- Automatic resampling to 16 kHz (Whisper requirement)
- Language selection: Portuguese, English, Spanish or auto-detect
- All transcriptions persisted in local SQLite database
- Full history with per-transcription status (PENDING → PROCESSING → COMPLETED / FAILED)
- Web interface embedded in the binary — no static file dependencies at runtime

---

## Tech Stack

| Layer | Technology | Description |
|---|---|---|
| Web Framework | **Axum 0.7** | Async HTTP framework built on Tokio |
| Async Runtime | **Tokio** | Async runtime for Rust |
| Database | **SQLite** (SQLx) | Local storage with connection pooling |
| Transcription | **Whisper.cpp** (`whisper-rs`) | Local speech recognition model |
| Error Handling | **thiserror** | Ergonomic error type derivation |
| Logging | **tracing** + **tracing-subscriber** | Structured logs with level filtering |
| Utilities | **uuid** | Unique ID generation per transcription |

---

## Prerequisites

- [Rust](https://rustup.rs) 1.75+
- [ffmpeg](https://ffmpeg.org/download.html) installed and on PATH (for non-WAV formats)
- Whisper model: `assets/ggml-base.bin`

```powershell
# Install ffmpeg via winget (Windows)
winget install ffmpeg

# Download Whisper base model (~140 MB)
mkdir assets
curl -L -o assets/ggml-base.bin `
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

---

## How to Run

```powershell
# Clone the repository
git clone https://github.com/Luisr-nunes/transcritor_de_audio.git
cd transcritor_de_audio

# Run
cargo run
```

> Server starts at `http://127.0.0.1:3000`. Open in browser to use the web interface.

### Production Build

```powershell
cargo build --release
# Binary at target/release/transcription-service.exe
```

---

## API

| Method | Route | Description |
|---|---|---|
| `GET` | `/` | Embedded web interface |
| `POST` | `/upload` | Submit audio for transcription |
| `GET` | `/transcriptions` | List all transcriptions |
| `GET` | `/transcriptions/:id` | Get transcription by ID |

### Upload example

```bash
curl -X POST http://127.0.0.1:3000/upload \
  -F "file=@audio.wav" \
  -F "language=pt"
```

### Response

```json
{
  "status": "completed",
  "transcription": "Ready, go ahead, it's recording."
}
```

---

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | `sqlite://<exe_dir>/transcriptions.db?mode=rwc` | SQLite database path |
| `WHISPER_MODEL` | `assets/ggml-base.bin` | Whisper model path |
| `RUST_LOG` | `info` | Log level (`debug`, `info`, `warn`, `error`) |

---

## Project Structure

```
transcritor_de_audio/
├── src/
│   ├── main.rs                          # Entry point, server initialization
│   ├── application/
│   │   └── transcribe.rs               # Main use case (transcription flow)
│   ├── domain/
│   │   └── transcription.rs            # Transcription entity + TranscriptionStatus
│   ├── infrastructure/
│   │   ├── api.rs                       # Axum router with all routes
│   │   ├── db.rs                        # SQLite pool and schema creation
│   │   ├── whisper.rs                   # Whisper integration + audio decoder
│   │   ├── http/
│   │   │   └── handler.rs              # HTTP handlers (upload, list, get)
│   │   └── repository/
│   │       └── transcription.rs        # Transcription CRUD (SQLx)
│   └── ui/
│       └── index.html                  # Web interface embedded via include_str!
├── assets/
│   └── ggml-base.bin                   # Whisper model (not versioned)
├── migrations/
│   └── 0001_create_transcriptions.sql  # Versioned SQL schema
└── Cargo.toml                          # Rust dependencies
```

---

## Developed by

**Luis Nunes** — [@Luisr-nunes](https://github.com/Luisr-nunes)
