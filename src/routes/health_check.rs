use axum::http::StatusCode;

#[axum::debug_handler]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
