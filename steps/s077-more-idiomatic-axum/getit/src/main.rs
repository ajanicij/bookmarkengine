use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct GetItResponse{
    message: String,
}

async fn get_it() -> Json<GetItResponse> {
    Json(GetItResponse {
        message: "Hello from Axum".to_string(),
    })
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
