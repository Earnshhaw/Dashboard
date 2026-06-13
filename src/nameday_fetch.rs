use axum::{Json, response::IntoResponse};
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

pub async fn nameday() -> impl IntoResponse {
    let client = Client::new();
    let Ok(response) = client.get(NAMEDAY_API_URL).send().await else {
        {
            eprintln!("Nameday API request failed");
            return Json(ProcessedResponse::default());
        }
    };

    let Ok(parsed) = response.json::<NamedayResponse>().await else {
        {
            eprintln!("Failed to parse Nameday API response");
            return Json(ProcessedResponse::default());
        }
    };

    let Some(parsed) = parsed.data.get("sk").cloned() else {
        {
            eprintln!("Nameday value not found in response");
            return Json(ProcessedResponse::default());
        }
    };

    let processed = ProcessedResponse { nameday: parsed };
    Json(processed)
}
