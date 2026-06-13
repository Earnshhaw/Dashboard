use axum::{Json, http::StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
struct NamedayResponse {
    success: bool,
    message: String,
    data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct ProcessedResponse {
    nameday: String,
}

static NAMEDAY_API_URL: &str = "https://nameday.abalin.net/api/V2/today/";

pub async fn nameday() -> Result<Json<ProcessedResponse>, StatusCode> {
    let client = Client::new();
    let response = client.get(NAMEDAY_API_URL).send().await.map_err(|e| {
        eprintln!("{e}");
        StatusCode::REQUEST_TIMEOUT
    })?;

    let parsed: NamedayResponse = response.json().await.map_err(|e| {
        eprintln!("{e}");
        StatusCode::REQUEST_TIMEOUT
    })?;

    let Some(parsed) = parsed.data.get("sk").cloned() else {
        {
            eprintln!("Nameday value not found in response");
            return Err(StatusCode::NOT_FOUND);
        }
    };

    Ok(Json(ProcessedResponse { nameday: parsed }))
}
