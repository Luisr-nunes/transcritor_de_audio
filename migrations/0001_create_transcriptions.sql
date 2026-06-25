-- Migration: 0001_create_transcriptions.sql
-- Cria a tabela principal de transcrições.

CREATE TABLE IF NOT EXISTS transcriptions (
    id        TEXT    PRIMARY KEY,
    filename  TEXT    NOT NULL,
    status    TEXT    NOT NULL DEFAULT 'PENDING',
    content   TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
