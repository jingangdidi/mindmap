use axum::{
    extract::OriginalUri,
    response::Html,
};
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
    DATA,
    INDEX,
    KATEX,
    DEFAULT_PAGE,
    parse_paras::PARAS,
};

/// Handler for `/` GET
pub async fn index(uri: OriginalUri) -> Html<String> {
    let uuid = Uuid::new_v4().to_string();
    event!(Level::INFO, "GET `{}`, create uuid: {}", uri.path(), &uuid);
    let data = DATA.read().unwrap();
    let pulldown = data.html_pulldown(&uuid);
    DEFAULT_PAGE
        .replace("127.0.0.1:8081", &format!("{}:{}", &PARAS.addr_str, PARAS.port))
        .replace("download/mindmap", &format!("download/{}", &uuid))
        .replace("locale: 'en'", &format!("locale: '{}'", PARAS.language))
        .replace("<option value='mindmap' selected>mindmap</option>", &format!("{}\n", pulldown))
        .replace("mindmap.png", &format!("{}.png", &uuid))
        .replace("const style = ``;", &format!("const style = `{}`;", INDEX))
        .replace("const katex = ``;", &format!("const katex = `{}`;", KATEX))
        .into()
}
