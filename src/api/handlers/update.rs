use std::collections::HashMap;

use axum::extract::{Query, OriginalUri};
use tracing::{event, Level};

use crate::DATA;

/// Handler for `/update` POST
pub async fn update(Query(params): Query<HashMap<String, String>>, uri: OriginalUri, body: String) -> () {
    // get uuid
    let uuid = match params.get("uuid") {
        Some(u) => {
            event!(Level::INFO, "{} POST `{}`", u, uri.path());
            u.clone()
        },
        None => {
            event!(Level::ERROR, "missing uuid POST `{}`", uri.path());
            return
        },
    };
    // get label
    let label = match params.get("label") {
        Some(l) => {
            if l.is_empty() {
                None
            } else {
                Some(l.clone())
            }
        },
        None => None,
    };
    // update loaded by body
    let mut data = DATA.write().unwrap();
    data.update_loaded_mindmap(uuid, body, label);
}
