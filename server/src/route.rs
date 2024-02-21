use axum::{routing::get, Router};

use crate::handler::{
    generate_elec1601, generate_math1002, show_elec1601_topics, show_math1002_topics,
    show_math1021_topics, show_units,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(show_units))
        .route("/elec1601", get(show_elec1601_topics))
        .route("/elec1601/:topic_id/:subtopic_id", get(generate_elec1601))
        .route("/math1061-1002", get(show_math1002_topics))
        .route(
            "/math1061-1002/:topic_id/:subtopic_id",
            get(generate_math1002),
        )
        .route("/math1061-1021", get(show_math1021_topics))
}
