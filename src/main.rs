use axum::{Router, routing::get};
use festivos_labs::services::free_days::REGLAS;

async fn health() -> &'static str {
    "OK"
}

async fn reglas() -> String {
    format!("{:#?}", REGLAS)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/reglas", get(reglas));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("escuchando en {addr}");

    axum::serve(listener, app).await.unwrap();
}
