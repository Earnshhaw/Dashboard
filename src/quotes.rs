use axum::Json;
use axum::http::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Deserialize, Serialize, Default)]
pub struct OuterQuotes {
    quotes: Vec<Quote>,
}

#[derive(Deserialize, Serialize)]
struct Quote {
    text: String,
    author: String,
}

pub async fn get_quotes() -> Result<Json<OuterQuotes>, StatusCode> {
    let quotes = fs::read_to_string("frontend/resources/quotes.json").map_err(|e| {
        eprintln!("Failed to read quotes.json: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let outer_quotes = serde_json::from_str::<OuterQuotes>(&quotes).map_err(|e| {
        eprintln!("Failed to parse quotes.json: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(outer_quotes))
}
