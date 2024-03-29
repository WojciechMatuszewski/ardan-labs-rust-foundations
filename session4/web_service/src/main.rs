use axum::response::Html;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(say_hello_json));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_text() -> Html<&'static str> {
    const HTML: &str = include_str!("hello.html");

    return Html(HTML);
}

async fn say_hello_json() -> Json<HelloJson> {
    return Json(HelloJson { message: "Foo".to_string() });
}