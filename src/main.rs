use axum::{Router, routing::get};
use tower_http::services::ServeDir;
mod nameday_fetch;
mod serve_schedule;
mod weather_fetch;
use crate::{nameday_fetch::nameday, quotes::get_quotes, weather_fetch::fetch_weather};
use serve_schedule::get_schedule;
mod posters_fetch;
use crate::posters_fetch::fetch_posters;
mod quotes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/schedule", get(get_schedule))
        .route("/api/weather", get(fetch_weather))
        .route("/api/nameday", get(nameday))
        .route("/api/posters", get(fetch_posters))
        .route("/api/quotes", get(get_quotes))
        .fallback_service(ServeDir::new("frontend"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
