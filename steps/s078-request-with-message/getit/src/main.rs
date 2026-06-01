use axum::{
    extract::Path,
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use tokio::net::TcpListener;

mod mylib;

#[derive(Serialize)]
struct MyFunResponse{
    response: String,
}

async fn myfun_handler(
    Path(message): Path<String>,
) -> Json<MyFunResponse> {
    // Calling a synchrounous library function.
    let result = mylib::myfun(message);

    Json(MyFunResponse {
        response: result,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/v1/myfun/{message}",
            get(myfun_handler),
        );

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("Listening on http://localhost:8000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
