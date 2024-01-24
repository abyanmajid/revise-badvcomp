use crate::{
    constants::UNITS,
    find_unit,
    types::{ErrorResponse, Unit},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn show_units() -> impl IntoResponse {
    const MESSAGE: &str = "Health Check!";

    (StatusCode::OK, MESSAGE)
}

pub async fn show_elec1601_topics() -> Result<Json<&'static Unit>, (StatusCode, Json<ErrorResponse>)>
{
    match find_unit("ELEC1601").await {
        Ok(unit) => Ok(Json(unit)),
        Err(error) => {
            let error_json = Json(ErrorResponse {
                status: error.status,
                message: error.message,
            });
            Err((StatusCode::from_u16(error.status).unwrap(), error_json))
        }
    }
}

pub async fn generate_elec1601(Path(id): Path<u32>) -> impl IntoResponse {}
