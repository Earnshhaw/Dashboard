use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub schedule: Arc<ScheduleResponse>,
}

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

pub async fn get_schedule(State(state): State<AppState>) -> impl IntoResponse {
    Json((*state.schedule).clone())
}
