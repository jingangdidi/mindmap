use std::io::Error;

use axum::{
    body::Body,
    extract::{OriginalUri, Path},
    http::{header, HeaderMap},
};
use tracing::{event, Level};

use crate::DATA;

/// Handler for `/download/:uuid` GET
/// download mindmap html file
pub async fn download(Path(uuid): Path<String>, uri: OriginalUri) -> (HeaderMap, Body) {
    // prepeare header
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap()); // value is `header::HeaderValue`, create: `HeaderValue::from_str("hello").unwrap()`
    // get content
    let data = DATA.read().unwrap();
    let html_str = match data.html_content(&uuid) {
        Some(content) => {
            event!(Level::INFO, "GET `{}`, download {}.html", uri.path(), uuid);
            headers.insert(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}.html\"", uuid).parse().unwrap());
            content
        },
        None => {
            event!(Level::INFO, "GET `{}`, unable to retrieve the relevant mindmap due to uuid {} not found", uri.path(), uuid);
            headers.insert(header::CONTENT_DISPOSITION, "attachment; filename=\"uuid_not_found.txt\"".parse().unwrap());
            format!("Unable to retrieve the relevant mindmap due to uuid {} not found.", uuid)
        },
    };
    // stream content
    let stream = async_stream::stream! {
        // https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/12
        let mut html_chunk = html_str.as_bytes().chunks(1024);
        while let Some(s) = html_chunk.next() {
            let tmp: Result<Vec<u8>, Error> = Ok(s.to_vec());
            yield tmp;
        }
        /*
        let tmp: Result<Vec<u8>, Error> = Ok(html_str.as_bytes().to_vec());
        yield tmp;
        */
    };
    // convert the `Stream` into an `axum::body::Body`
    let body = Body::from_stream(stream); // v0.7 change `StreamBody::new(stream);` to `Body::from_stream(stream)`
    (headers, body)
}
