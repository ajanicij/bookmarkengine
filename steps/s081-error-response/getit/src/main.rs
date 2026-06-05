mod mylib;

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MyFunParams {
    name: String,
    greeting: String,
}

#[derive(Debug)]
enum AppError {
    InvalidName,
    InternalError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InvalidName => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "name xyz is not allowed".to_string(),
                }),
            )
                .into_response(),

            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "internal server error".to_string(),
                }),
            )
                .into_response(),
        }
    }
}

#[derive(Serialize)]
struct SuccessResponse {
    result: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/myfun", get(myfun_handler))
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:8000");

    axum::serve(listener, app).await.unwrap();
}

async fn myfun_handler(
    Query(params): Query<MyFunParams>,
) -> Result<Json<SuccessResponse>, AppError> {
    let name = params.name;
    let greeting = params.greeting;

    let result = tokio::task::spawn_blocking(move || {
        mylib::myfun(name, greeting)
    })
    .await
    .map_err(|_| AppError::InternalError)?;

    match result {
        Ok(value) => Ok(Json(SuccessResponse { result: value })),
        Err(mylib::MyLibError::InvalidName) => Err(AppError::InvalidName),
    }
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "invalid endpoint".to_string(),
        }),
    )
}