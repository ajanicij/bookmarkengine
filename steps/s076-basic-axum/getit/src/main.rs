use axum::{
    routing::get,
    Json,
    Router,
};
use serde_json::json;
use tokio::net::TcpListener;

async fn get_it() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello from Axum"
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/getit", get(get_it));

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("Listening on http://localhost:8000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
