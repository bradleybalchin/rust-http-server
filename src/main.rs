mod api;
mod handlers;
mod models;


use axum::{
    routing::get,
    Router,
    middleware
};
use tower_http::services::{ServeDir, ServeFile};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use uuid;


#[tokio::main]
async fn main() {

    // TODO: move index to askama handler
    // TODO: cookie management
    // TODO: must have valid session cookie to access api or files, can only see login page
    // Redirect to login page if no valid session cookie
    // TODO : Admin mode? (nested in logged in)


    //let admin_protected_routes = Router::new();
    //let user_protected_routes = Router::new();

    // router for webserver
    let app = Router::new()
    .route("/login", get(handlers::auth::login_page)
                                        .post(handlers::auth::login))                                  
    .nest("/api/files", api::files::router())
    .fallback_service(
    ServeDir::new("public")
        .not_found_service(ServeFile::new("public/404.html")),
    )
    .layer(CookieManagerLayer::new());
        

    // run webserver
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
