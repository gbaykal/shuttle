use actix_web::{web,  get, HttpResponse};

#[get("/1/{num1}/{num2}")]
async fn calculate_xor_pow3(info: web::Path<(u32, u32)>) -> HttpResponse {
    let (num1, num2) = info.into_inner();
    let result = (num1 ^ num2).pow(3);
    HttpResponse::Ok().json(result)
}

