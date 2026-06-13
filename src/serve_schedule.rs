use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Clone, Deserialize, Default)]
struct LessonBlock {
    r#type: String,
    name: String,
    start: String,
    end: String,
}

#[derive(Serialize, Clone, Deserialize, Default)]
pub struct ScheduleResponse {
    schedule: Vec<LessonBlock>,
}

pub async fn get_schedule() -> Result<Json<ScheduleResponse>, StatusCode> {
    let file = fs::read_to_string("frontend/resources/schedule.json").map_err(|e| {
        eprintln!("Failed to read schedule.json: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let schedule: ScheduleResponse = serde_json::from_str(&file).map_err(|e| {
        eprintln!("Failed to parse schedule.json: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(schedule))
}
