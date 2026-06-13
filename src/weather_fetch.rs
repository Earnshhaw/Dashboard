use axum::Json;
use axum::http::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct RawWeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_units: current_units,
    pub current: current,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub struct current_units {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub relative_humidity_2m: String,
    pub apparent_temperature: String,
    pub wind_speed_10m: String,
    pub precipitation: String,
    pub cloud_cover: String,
    pub weathercode: String,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub struct current {
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f32,
    pub relative_humidity_2m: f32,
    pub apparent_temperature: f32,
    pub wind_speed_10m: f32,
    pub precipitation: f32,
    pub cloud_cover: f32,
    pub weathercode: i32,
}

impl AsRef<std::path::Path> for ProcessedWeatherResponse {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new("")
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ProcessedWeatherResponse {
    temperature: f32,
    humidity: f32,
    apparent_temperature: f32,
    wind_speed: f32,
    precipitation: f32,
    cloud_cover: f32,
    weather_code: i32,
}

static WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=48.7164&longitude=21.2611&current=temperature_2m,relative_humidity_2m,apparent_temperature,wind_speed_10m,precipitation,cloud_cover,weathercode&timezone=auto";

pub async fn fetch_weather() -> Result<Json<ProcessedWeatherResponse>, StatusCode> {
    let client = Client::new();
    let response = client.get(WEATHER_API_URL).send().await.map_err(|e| {
        eprintln!("Weather API request failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let body = response.json::<RawWeatherResponse>().await.map_err(|e| {
        eprintln!("Failed to parse Weather API response: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let processed = ProcessedWeatherResponse {
        temperature: body.current.temperature_2m,
        humidity: body.current.relative_humidity_2m,
        apparent_temperature: body.current.apparent_temperature,
        wind_speed: body.current.wind_speed_10m,
        precipitation: body.current.precipitation,
        cloud_cover: body.current.cloud_cover,
        weather_code: body.current.weathercode,
    };
    Ok(Json(processed))
}
