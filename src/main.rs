mod api;
mod handlers;
mod models;
mod middleware;


use axum::{
    routing::get,
    Router,
    middleware as axum_middleware
};
use tower_http::services::{ServeDir, ServeFile};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use uuid;


#[tokio::main]
async fn main() {

    // TODO: db connection
    // TODO: cookie management
    // TODO: must have valid session cookie to access api or files, can only see login page
    // TODO : Admin mode? (nested in logged in)


    //let admin_protected_routes = Router::new();
    //let user_protected_routes = Router::new();

    //must be logged in as user 
    let user_protected_routes = Router::new()
    //index
    .route("/", get(handlers::home::index))

    //file api
    .nest("/api/files", api::files::router())

    // user authenitcation middleware
    .layer(axum_middleware::from_fn(middleware::user::auth_user));

    // router for webserver
    let app = Router::new()
    //auth
    .route("/login", get(handlers::auth::login_page)
                                        .post(handlers::auth::login))     

    .merge(user_protected_routes)

    //404 fallback
    .fallback_service(
    ServeDir::new("public")
        .not_found_service(ServeFile::new("public/404.html")),
    )
    .layer(CookieManagerLayer::new());
        

    // run webserver
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
