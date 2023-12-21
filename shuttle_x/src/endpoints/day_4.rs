use actix_web::{web, guard, App, HttpServer, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}


#[post("/4/strength")]
async fn calculate_combined_strength(data: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let combined_strength: u32 = data.iter().map(|r| r.strength).sum();
    HttpResponse::Ok().json(combined_strength)
}