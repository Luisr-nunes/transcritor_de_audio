async fn upload_handler(
    State(pool): State<Arc<SqlitePool>>,
    mut multipart: Multipart
) -> &'static str {
    
    sqlx::query("INSERT INTO transcriptions (id, filename, status, content) VALUES (?, ?, ?, ?)")
        .bind("uuid-gerado-aqui")
        .bind("nome_do_arquivo.mp4")
        .bind("PENDING")
        .bind(None::<String>)
        .execute(&*pool)
        .await
        .unwrap();

    "Arquivo processado e registrado!"
}