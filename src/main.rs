mod api;
mod handlers;

use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};


#[tokio::main]
async fn main() {
    // router for webserver
    let app = Router::new()
    .nest("/api/files", api::files::router())
    .fallback_service(
    ServeDir::new("public")
        .not_found_service(ServeFile::new("public/404.html")),
    );
        



    // run webserver
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
