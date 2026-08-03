mod api;
mod handlers;
mod models;
mod middleware;
mod database;

use axum::{
    Router, middleware as axum_middleware, routing::{get, post}
};
use tower_http::services::{ServeDir, ServeFile};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use uuid;

use crate::models::state::AppState;


#[tokio::main]
async fn main() {

    // establish db connection
    let db = database::connection::connect()
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    // run migrations
    sqlx::migrate!("./migrations")
    .run(&db)
    .await.expect("Could not perform migrations");

    //create state
    let state = models::state::AppState { db };


    // TODO: cookie management
    // TODO: must have valid session cookie to access api or files, can only see login page
    // TODO : Admin mode? (nested in logged in)


    //let admin_protected_routes = Router::new();

    //must be logged in as user 
    let user_protected_routes = Router::<AppState>::new()
        .route("/", get(handlers::home::index))     //index
        .nest("/api/files", api::files::router())     //file api
        .layer(axum_middleware::from_fn(middleware::user::auth_user));     // user authenitcation middleware

    // router for webserver
    let app = Router::<AppState>::new()
        .route("/login", get(handlers::auth::login_page))//auth                                  .post(handlers::auth::login))     
        .route("/logout", post(handlers::auth::logout))
        .merge(user_protected_routes)

        .fallback_service(
        ServeDir::new("public")
            .not_found_service(ServeFile::new("public/404.html")),    //404 fallback
        )
        .layer(CookieManagerLayer::new())
        .with_state(state); //database connection
        

    // run webserver using tokio async runtime
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
