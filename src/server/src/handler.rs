use super::{create_error_response, find_unit, map_units_to_summary};
use crate::constants::UNITS;
use crate::elec1601;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::Value;
use tracing::{debug, error};

pub async fn show_units() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    debug!("Request received to show all units");

    let unit_summaries = map_units_to_summary(&*UNITS);

    if unit_summaries.is_empty() {
        debug!("No units found for show_units request");
        return Err(create_error_response(
            StatusCode::NOT_FOUND,
            String::from("No units found"),
        ));
    }

    debug!("Unit summaries successfully retrieved");
    Ok((StatusCode::OK, Json(unit_summaries)))
}

pub async fn show_elec1601_topics() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    debug!("Request received to show topics for ELEC1601");

    match find_unit("ELEC1601").await {
        Ok(unit) => {
            debug!("ELEC1601 unit successfully found and retrieved");
            println!("{:?}", unit);
            Ok((StatusCode::OK, Json(unit)))
        }
        Err(error) => {
            error!(
                "Error occurred while finding ELEC1601 unit: {}",
                error.message
            );
            Err(create_error_response(
                StatusCode::from_u16(error.status).unwrap(),
                String::from(error.message),
            ))
        }
    }
}

pub async fn generate_elec1601(
    Path((topic_id, subtopic_id)): Path<(u32, u32)>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    debug!("Handling request to generate problem for id: {}", topic_id);

    match elec1601::generate_problem(topic_id, subtopic_id) {
        Some(problem) => {
            debug!("Successfully generated ELEC1601 base encoding problem");
            Ok((StatusCode::OK, Json(problem)))
        }
        None => {
            debug!(
                "Could not find topic with ID: {}, and sub-topic ID: {}",
                topic_id, subtopic_id
            );
            let error_message = format!(
                "Could not find topic with ID: {}, and sub-topic ID: {}",
                topic_id, subtopic_id
            );
            Err(create_error_response(StatusCode::NOT_FOUND, error_message))
        }
    }
}
