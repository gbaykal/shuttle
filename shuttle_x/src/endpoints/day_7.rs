use actix_web::{web, HttpResponse, Responder, get, App, HttpRequest};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};

#[get("/7/decode")]
    async fn decode_cookie(req: HttpRequest) -> HttpResponse {
    let buffer = req.cookie("recipe").unwrap();
    let decoded_bytes = general_purpose::STANDARD.decode(buffer.value()).unwrap_or_default();
    let decoded_str = String::from_utf8_lossy(&decoded_bytes);
    println!("{:?}",decoded_str);
    let decoded_recipe: Value = serde_json::from_str(&decoded_str).unwrap_or_default(); //no need to struct bc of that json conv.

    return HttpResponse::Ok().json(decoded_recipe);
}