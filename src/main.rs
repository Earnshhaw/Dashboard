use axum::{Router, routing::get};
use tower_http::services::ServeDir;
mod nameday_fetch;
mod posters_fetch;
mod weather_fetch;
use crate::{nameday_fetch::nameday, posters_fetch::fetch_posters, weather_fetch::fetch_weather};

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service(
            "/api/schedule",
            ServeDir::new("frontend/resources/schedule.json"),
        )
        .nest_service(
            "/api/quotes",
            ServeDir::new("frontend/resources/quotes.json"),
        )
        .route("/api/weather", get(fetch_weather))
        .route("/api/nameday", get(nameday))
        .route("/api/posters", get(fetch_posters))
        .fallback_service(ServeDir::new("frontend"));

    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    println!("Listening on http://{}", ADDRESS);

    axum::serve(listener, app).await.unwrap();
}
