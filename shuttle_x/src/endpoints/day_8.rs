use actix_web::{web, HttpResponse, Responder, get, App, HttpRequest};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
struct PokemonInfo {
    weight : i32
}

#[get("/8/weight/{pokedex_number}")]
async fn pokemon_weight (_req: HttpRequest, path: web::Path<i32> ) -> HttpResponse {
    let pokemon_id = path.into_inner();
    let pokemon_info = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}",pokemon_id)).await.unwrap().text().await.unwrap();
    let pokemon_info_agent:PokemonInfo = serde_json::from_str(&pokemon_info).unwrap(); 
    HttpResponse::Ok().json((pokemon_info_agent.weight as f64)/10.0)
}