use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Debug)]
pub struct Poster {
    title: String,
    path: String,
}

pub async fn fetch_posters() -> Result<Json<Vec<Poster>>, StatusCode> {
    let mut posters: Vec<Poster> = Vec::new();

    let dir = fs::read_dir("frontend/posters").map_err(|e| {
        eprintln!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    for entry in dir {
        let entry = entry.map_err(|e| {
            eprintln!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

        let path = entry.path();
        let edited_path = path.clone().to_string_lossy().replace("\\", "/");

        if let Some(ext) = path.extension() {
            if ext == "jpg" || ext == "png" || ext == "jpeg" {
                posters.push(Poster {
                    title: path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string(),
                    path: edited_path,
                });
            }
        }
    }
    Ok(Json(posters))
}
