use axum::{routing::get, Router};

use crate::handler::{show_elec1601_topics, show_units};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(show_units))
        .route("/elec1601", get(show_elec1601_topics))
        .route("/elec1601/:id", get(show_units))
}
