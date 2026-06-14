use axum::{Json, http::StatusCode};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(unused)]
#[derive(Debug, Clone, Deserialize)]
struct NamedayResponse {
    #[serde(default)]
    data: HashMap<String, String>,
}

static NAMEDAY_API_URL: &str = "https://nameday.abalin.net/api/V2/today/";

pub async fn nameday() -> Result<Json<String>, StatusCode> {
    let client = Client::new();

    let response = client.get(NAMEDAY_API_URL).send().await.map_err(|e| {
        eprintln!("{e}");
        StatusCode::REQUEST_TIMEOUT
    })?;

    let parsed: NamedayResponse = response.json().await.map_err(|e| {
        eprintln!("Deserialization error in nameday: {e}");
        StatusCode::REQUEST_TIMEOUT
    })?;

    let Some(parsed) = parsed.data.get("sk").cloned() else {
        {
            eprintln!("Nameday value not found in response");
            return Err(StatusCode::NOT_FOUND);
        }
    };

    Ok(Json(parsed))
}
