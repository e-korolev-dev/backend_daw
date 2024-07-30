use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Track {
    id: u32,
    name: String,
    duration: u32,
}

async fn get_tracks() -> HttpResponse {
    let tracks = vec![
        Track { id: 1, name: "Track 1".to_string(), duration: 300 },
        Track { id: 2, name: "Track 2".to_string(), duration: 250 },
    ];
    HttpResponse::Ok().json(tracks)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tracks").route(web::get().to(get_tracks)));
}
