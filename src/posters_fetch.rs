use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Debug)]
struct Poster {
    title: String,
    path: String,
}

pub async fn fetch_posters() -> impl IntoResponse {
    let mut posters: Vec<Poster> = Vec::new();

    let dir = match fs::read_dir("frontend/posters") {
        Ok(dir) => dir,
        Err(_) => return Json(posters),
    };

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

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
    Json(posters)
}
