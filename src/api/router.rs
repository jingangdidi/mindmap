use axum::routing::{get, post};
use axum::Router;

use crate::api::handlers::{
    index::index,
    update::update,
    previous::previous,
    download::download,
    fallback::fallback,
};

/// https://github.com/tokio-rs/axum/blob/main/examples/templates/src/main.rs
/// https://dev.to/shuttle_dev/building-a-simple-web-server-in-rust-5c57
/// https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs
/// https://matze.github.io/axum-notes/notes/templating/with_askama/index.html
pub fn configure() -> Router {
    Router::new()
        .route("/", get(index)) // GET `/`
        .route("/update", post(update)) // POST `/update`
        .route("/previous", get(previous)) // GET `/previous`
        .route("/download/:uuid", get(download)) // GET `/download/:uuid`
        .fallback(fallback) // not match any router
}
