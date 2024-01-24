pub mod constants;
pub mod elec1601;
pub mod handler;
pub mod route;
pub mod types;

use axum::http::StatusCode;
use constants::UNITS;
use types::{ErrorResponse, Unit};

async fn find_unit(unit_name: &str) -> Result<&'static Unit, ErrorResponse> {
    UNITS
        .iter()
        .find(|&u| u.unit == unit_name)
        .ok_or(ErrorResponse {
            status: StatusCode::NOT_FOUND.as_u16(),
            message: "Unit not found",
        })
}
