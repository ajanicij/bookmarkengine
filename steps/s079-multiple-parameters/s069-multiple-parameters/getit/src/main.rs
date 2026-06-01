use axum::{
    extract::Query,
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod mylib;

#[derive(Deserialize)]
struct MyFunQuery {
    name: String,
    greeting: String,
}

#[derive(Serialize)]
struct MyFunResponse{
    response: String,
}

async fn myfun_handler(
    Query(params): Query<MyFunQuery>,
) -> Json<MyFunResponse> {
    // Calling a synchrounous library function.
    let result = mylib::myfun(
        params.name,
        params.greeting,
    );

    Json(MyFunResponse {
        response: result,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/v1/myfun",
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
