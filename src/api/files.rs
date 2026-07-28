use crate::handlers;

use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

// route to handler
pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::files::list))
}