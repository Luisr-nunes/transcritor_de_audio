# Transcritor de Áudio

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![Tokio](https://img.shields.io/badge/Tokio-4A4A4A?style=flat-square) ![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white) ![Whisper](https://img.shields.io/badge/Whisper-412991?style=flat-square&logo=openai&logoColor=white) ![FFmpeg](https://img.shields.io/badge/FFmpeg-007808?style=flat-square&logo=ffmpeg&logoColor=white) ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white)

[Read in English](README.en.md)

Serviço de transcrição de áudio em Rust que roda inteiramente na sua máquina: o áudio é processado localmente pelo modelo Whisper, sem enviar dados para a nuvem. Expõe uma API REST e traz uma interface web embutida no próprio binário.

## Funcionalidades

- Upload de áudio pela interface web ou por `multipart/form-data`.
- Formatos WAV, MP3, MP4, OGG, FLAC e WebM; o que não for WAV é convertido com `ffmpeg`.
- Decodificação WAV nativa: PCM de 8, 16, 24 e 32 bits, ponto flutuante de 32 e 64 bits, mono e estéreo.
- Reamostragem automática para 16 kHz, exigida pelo Whisper.
- Idioma em português, inglês, espanhol ou detecção automática.
- Histórico em SQLite com status por transcrição: `PENDING`, `PROCESSING`, `COMPLETED` ou `FAILED`.
- Interface web embutida com `include_str!`, sem arquivos estáticos em tempo de execução.

## Tecnologias

| Camada | Tecnologia |
| --- | --- |
| Web | Axum 0.7, Tokio, Tower |
| Banco | SQLite via SQLx 0.7 |
| Transcrição | Whisper.cpp via `whisper-rs` 0.16 |
| Conversão de áudio | `ffmpeg` (processo externo) |
| Erros e logs | `thiserror`, `tracing`, `tracing-subscriber` |

## Arquitetura

```
src/
├── main.rs                         # inicialização do servidor (127.0.0.1:3000)
├── domain/transcription.rs         # entidade Transcription e status
├── application/transcribe.rs       # caso de uso de transcrição
├── infrastructure/
│   ├── api.rs                      # rotas Axum
│   ├── db.rs                       # pool SQLite e schema
│   ├── whisper.rs                  # Whisper, decodificador WAV e ffmpeg
│   ├── http/handler.rs             # upload, listagem e consulta
│   └── repository/transcription.rs # persistência
└── ui/index.html                   # interface embutida
migrations/0001_create_transcriptions.sql
```

## Pré-requisitos

- Rust 1.75+
- CMake e um compilador C/C++, usados pelo `whisper-rs` para compilar o Whisper.cpp
- `ffmpeg` no PATH, para formatos além de WAV
- Modelo Whisper em `assets/ggml-base.bin` (cerca de 140 MB, não versionado)

```bash
mkdir assets
curl -L -o assets/ggml-base.bin \
  https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
```

No Windows, o `ffmpeg` pode ser instalado com `winget install ffmpeg`.

## Como executar

```bash
git clone https://github.com/Luisr-nunes/transcritor_de_audio.git
cd transcritor_de_audio
cargo run            # ou: make run
```

Acesse `http://127.0.0.1:3000`. Para o build otimizado, `cargo build --release` gera `target/release/transcription-service` (`.exe` no Windows).

O manifesto do projeto está salvo como `cargo.toml`, em minúsculas. No Windows isso funciona, mas no Linux e no macOS o Cargo exige `Cargo.toml`; renomeie o arquivo antes de compilar nesses sistemas.

## API

| Método | Rota | Descrição |
| --- | --- | --- |
| GET | `/` | Interface web |
| POST | `/upload` | Envia o áudio (campos `file` e `language`) |
| GET | `/transcriptions` | Lista as transcrições |
| GET | `/transcriptions/:id` | Consulta uma transcrição |

```bash
curl -X POST http://127.0.0.1:3000/upload -F "file=@audio.wav" -F "language=pt"
```

```json
{ "status": "completed", "transcription": "Pronto, pode falar, tá gravando." }
```

## Variáveis de ambiente

| Variável | Padrão | Descrição |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://<pasta do executável>/transcriptions.db?mode=rwc` | Banco SQLite |
| `WHISPER_MODEL` | `assets/ggml-base.bin` | Caminho do modelo |
| `RUST_LOG` | `info` | Nível de log |

## Autor

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)
