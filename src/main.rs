mod domain;
mod application;
mod infrastructure;

#[tokio::main]
async fn main() {
    let app = infrastructure::api::app().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Servidor rodando em http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
