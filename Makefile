.PHONY: build run dev test clean

# Compila em modo debug
build:
	cargo build

# Compila e executa
run:
	cargo run

# Executa com logs detalhados
dev:
	RUST_LOG=debug cargo run

# Roda os testes
test:
	cargo test

# Remove artefatos de build
clean:
	cargo clean
	rm -f transcriptions.db

# Baixa o modelo Whisper base (requer wget)
download-model:
	mkdir -p assets
	wget -O assets/ggml-base.bin \
		https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
