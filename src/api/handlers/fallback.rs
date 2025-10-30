use axum::http::StatusCode;

/// Handler for any request that fails to match the router routes
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    //event!(Level::INFO, "fails to match any route");
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}
