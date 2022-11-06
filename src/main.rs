use axum::{body, extract::Path, response::Response, routing::get, Router};
use http::{header, StatusCode};
use rust_embed::RustEmbed;
use std::process::Command;

#[derive(RustEmbed)]
#[folder = "public"]
struct StaticAssets;

#[tokio::main]
async fn main() {
    Command::new("zola")
        .arg("build")
        .status()
        .expect("could not build site");

    let router = Router::new().route("/*path", get(static_files));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn static_files(Path(path): Path<String>) -> Response {
    let content = StaticAssets::get(if path.ends_with('/') {
        &(path.clone() + "index.html")
    } else {
        &path
    });
    // .ok_or_else(|| StatusCode::NOT_FOUND);

    let body = body::boxed(body::Full::from(content.unwrap().data));
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}
