pub mod cli;
pub mod constants;
pub mod elec1601;
pub mod handler;
pub mod logger;
pub mod route;
pub mod types;

use axum::{http::StatusCode, Json};
use constants::UNITS;
use serde_json::{json, Value};
use tracing::trace;
use types::{ErrorResponse, Unit, UnitSummary};

pub async fn find_unit(unit_name: &str) -> Result<&'static Unit, ErrorResponse> {
    trace!("Attempting to find unit with name: {}", unit_name);
    UNITS.iter().find(|&u| u.unit == unit_name).ok_or_else(|| {
        trace!("Unit not found: {}", unit_name);
        ErrorResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            message: "Unit not found",
        }
    })
}

pub fn map_units_to_summary(units: &'static [Unit]) -> Vec<UnitSummary> {
    trace!("Mapping {} units to a UnitSummary instance...", units.len());
    units
        .iter()
        .map(|u| {
            trace!("Processing unit: {}", u.unit);
            UnitSummary {
                unit: &u.unit,
                topics: u.topics.len(),
            }
        })
        .collect()
}

pub fn create_error_response(status: StatusCode, message: String) -> (StatusCode, Json<Value>) {
    trace!("Creating error response: {}, {}", status, message);
    let error_response = json!({
        "status": status.as_u16(),
        "message": message
    });
    (status, Json(error_response))
}
