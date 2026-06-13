use axum::{Json, response::IntoResponse};
use serde::Deserialize;
use serde::Serialize;
use std::fs;
#[derive(Deserialize, Serialize, Default)]
struct OuterQuotes {
    quotes: Vec<Quote>,
}

#[derive(Deserialize, Serialize)]
struct Quote {
    text: String,
    author: String,
}

pub async fn get_quotes() -> impl IntoResponse {
    let Ok(quotes) = fs::read_to_string("frontend/dist/quotes.json") else {
        return Json::default();
    };
    let Ok(outer_quotes) = serde_json::from_str::<OuterQuotes>(&quotes) else {
        return Json::default();
    };

    Json(outer_quotes)
}
