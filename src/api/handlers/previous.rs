use std::collections::HashMap;

use axum::{
    extract::{Query, OriginalUri},
    response::Html,
};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    DEFAULT_PAGE,
    INDEX,
    KATEX,
    DATA,
    parse_paras::PARAS,
};

/// Handler for `/previous` GET
pub async fn previous(Query(params): Query<HashMap<String, String>>, uri: OriginalUri) -> Html<String> {
    let (content_label, pulldown, uuid): (Option<(String, Option<String>)>, String, String) = match params.get("uuid") {
        Some(uuid) => {
            let mut data = DATA.write().unwrap();
            let (content_label, pulldown) = data.get_local_mindmap(uuid);
            if content_label.is_none() {
                event!(Level::INFO, "GET `{}`, redirect to {}, but no such uuid in local, create new mindmap", uri.path(), uuid);
            } else {
                event!(Level::INFO, "GET `{}`, redirect to {}", uri.path(), uuid);
            }
            (content_label, pulldown, uuid.clone())
        },
        None => {
            let uuid = Uuid::new_v4().to_string();
            event!(Level::INFO, "GET `{}`, missing uuid when redirect, create new uuid {}", uri.path(), &uuid);
            let data = DATA.read().unwrap();
            let pulldown = data.html_pulldown(&uuid);
            (None, pulldown, uuid)
        },
    };
    match content_label {
        Some((content, label)) => {
            let mut html = DEFAULT_PAGE
                .replace("127.0.0.1:8081", &format!("{}:{}", &PARAS.addr_str, PARAS.port))
                .replace("download/mindmap", &format!("download/{}", &uuid))
                .replace("<option value='mindmap' selected>mindmap</option>", &format!("{}\n", pulldown))
                .replace("mindmap.png", &format!("{}.png", &uuid))
                .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                .replace("MindElixir.new('root')", &format!("JSON.parse('{}')", &content));
            if PARAS.language != "en" {
                html = html.replace("locale: 'en'", &format!("locale: '{}'", PARAS.language));
            }
            if let Some(l) = label {
                html = html.replace("placeholder='mindmap label'>", &format!("placeholder='{}'>", l));
            }
            html.into()
        },
        None => {
            DEFAULT_PAGE
                .replace("127.0.0.1:8081", &format!("{}:{}", &PARAS.addr_str, PARAS.port))
                .replace("download/mindmap", &format!("download/{}", &uuid))
                .replace("locale: 'en'", &format!("locale: '{}'", PARAS.language))
                .replace("<option value='mindmap' selected>mindmap</option>", &format!("{}\n", pulldown))
                .replace("mindmap.png", &format!("{}.png", &uuid))
                .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
                .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
                .into()
        },
    }
}
