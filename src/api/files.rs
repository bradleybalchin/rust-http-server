use crate::{handlers, models::state::AppState};

use axum::{
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

// route to handler
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", get(handlers::files::list))
}