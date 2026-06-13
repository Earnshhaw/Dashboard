use axum::{Json, response::IntoResponse};
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

pub async fn get_schedule() -> impl IntoResponse {
    let file = fs::read_to_string("frontend/dist/schedule.json").unwrap();
    let schedule: ScheduleResponse = serde_json::from_str(&file).unwrap_or_else(|_| {
        eprintln!("Failed to parse schedule.json, using default fallback");
        ScheduleResponse::default()
    });
    Json(schedule)
}
